pub mod alias;
pub mod brush;
pub mod deferred;
pub mod particle;
pub mod postprocess;
pub mod sprite;

use std::{cell::RefCell, mem::size_of};

use crate::{
    client::{
        unit::particle::Particle,
        render::{
            pipeline::{Pipeline, PushConstantUpdate},
            uniform::{DynamicUniformBufferBlock, UniformArrayFloat, UniformBool},
            world::{
                alias::{AliasPipeline, AliasRenderer},
                brush::{BrushPipeline, BrushRenderer, BrushRendererBuilder},
                sprite::{SpritePipeline, SpriteRenderer},
            },
            GraphicsState, DEPTH_ATTACHMENT_FORMAT, DIFFUSE_ATTACHMENT_FORMAT,
            LIGHT_ATTACHMENT_FORMAT, NORMAL_ATTACHMENT_FORMAT,
        },
        ClientUnit,
    },
    common::{
        console::CvarRegistry,
        engine,
        math::Angles,
        model::{Model, ModelKind},
        sprite::SpriteKind,
        util::any_as_bytes, gamestate::component::physics::PhysicsComponent,
    },
};

use bevy_ecs::{query::{QueryState, QueryIter, WorldQuery}, prelude::Bundle, world::World as BevyWorld} ;
use bumpalo::Bump;
use cgmath::{Euler, InnerSpace, Matrix4, SquareMatrix as _, Vector3, Vector4, Deg};
use chrono::Duration;

use super::{RenderQuery,RenderQueryItem};

lazy_static! {
    static ref BIND_GROUP_LAYOUT_DESCRIPTOR_BINDINGS: [Vec<wgpu::BindGroupLayoutEntry>; 2] = [
        vec![
            wgpu::BindGroupLayoutEntry {
                binding:0,
                visibility:wgpu::ShaderStages::all(),
                ty:wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:
                        std::num::NonZeroU64::new(size_of::<FrameUniforms>() as u64)
                },
                count:None,
            },
        ],
        vec![
            // transform matrix
            // TODO: move this to push constants once they're exposed in wgpu
            wgpu::BindGroupLayoutEntry {
                binding:0,
                visibility:wgpu::ShaderStages::VERTEX,
                ty:wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size:
                        std::num::NonZeroU64::new(size_of::<EntityUniforms>() as u64)
                },
                count:None,
            },
            // diffuse and fullbright sampler
            wgpu::BindGroupLayoutEntry {
                binding:1,
                visibility:wgpu::ShaderStages::FRAGMENT,
                ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count:None,
            },
            // lightmap sampler
            wgpu::BindGroupLayoutEntry {
                binding:2,
                visibility:wgpu::ShaderStages::FRAGMENT,
                ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count:None,
            },
        ],
    ];

    pub static ref BIND_GROUP_LAYOUT_DESCRIPTORS: [wgpu::BindGroupLayoutDescriptor<'static>; 2] = [
        // group 0: updated per-frame
        wgpu::BindGroupLayoutDescriptor {
            label: Some("per-frame bind group"),
            entries: &BIND_GROUP_LAYOUT_DESCRIPTOR_BINDINGS[0],
        },
        // group 1: updated per-entity
        wgpu::BindGroupLayoutDescriptor {
            label: Some("brush per-entity bind group"),
            entries: &BIND_GROUP_LAYOUT_DESCRIPTOR_BINDINGS[1],
        },
    ];
}





struct WorldPipelineBase;

impl Pipeline for WorldPipelineBase {
    type VertexPushConstants = ();
    type SharedPushConstants = ();
    type FragmentPushConstants = ();

    fn name() -> &'static str {
        "world"
    }

    fn vertex_shader() -> &'static str {
        ""
    }

    fn fragment_shader() -> &'static str {
        ""
    }

    fn bind_group_layout_descriptors() -> Vec<wgpu::BindGroupLayoutDescriptor<'static>> {
        // TODO
        vec![]
    }

    fn primitive_state() -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        }
    }

    fn color_target_states() -> Vec<Option<wgpu::ColorTargetState>> {
        vec![
            // diffuse attachment
            Some(wgpu::ColorTargetState {
                format: DIFFUSE_ATTACHMENT_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }),
            // normal attachment
            Some(wgpu::ColorTargetState {
                format: NORMAL_ATTACHMENT_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }),
            // light attachment
            Some(wgpu::ColorTargetState {
                format: LIGHT_ATTACHMENT_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }),
        ]
    }

    fn depth_stencil_state() -> Option<wgpu::DepthStencilState> {
        Some(wgpu::DepthStencilState {
            format: DEPTH_ATTACHMENT_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState {
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: wgpu::DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        })
    }

    fn vertex_buffer_layouts() -> Vec<wgpu::VertexBufferLayout<'static>> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BindGroupLayoutId {
    PerFrame = 0,
    PerEntity = 1,
    PerTexture = 2,
    PerFace = 3,
}

pub struct Camera {
    origin: Vector3<f32>,
    angles: Angles,
    view: Matrix4<f32>,
    view_projection: Matrix4<f32>,
    projection: Matrix4<f32>,
    inverse_projection: Matrix4<f32>,
    clipping_planes: [Vector4<f32>; 6],
}

impl Camera {
    pub fn new(origin: Vector3<f32>, angles: Angles, projection: Matrix4<f32>) -> Camera {
        // convert coordinates
        let converted_origin = Vector3::new(-origin.y, origin.z, -origin.x);

        // translate the world by inverse of camera position
        let translation = Matrix4::from_translation(-converted_origin);
        let rotation = angles.mat4_wgpu();
        let view = rotation * translation;
        let view_projection = projection * view;

        // see https://www.gamedevs.org/uploads/fast-extraction-viewing-frustum-planes-from-world-view-projection-matrix.pdf
        let clipping_planes = [
            // left
            view_projection.w + view_projection.x,
            // right
            view_projection.w - view_projection.x,
            // bottom
            view_projection.w + view_projection.y,
            // top
            view_projection.w - view_projection.y,
            // near
            view_projection.w + view_projection.z,
            // far
            view_projection.w - view_projection.z,
        ];

        Camera {
            origin,
            angles,
            view,
            view_projection,
            projection,
            inverse_projection: projection.invert().unwrap(),
            clipping_planes,
        }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }

    pub fn angles(&self) -> Angles {
        self.angles
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view
    }

    pub fn view_projection(&self) -> Matrix4<f32> {
        self.view_projection
    }

    pub fn projection(&self) -> Matrix4<f32> {
        self.projection
    }

    pub fn inverse_projection(&self) -> Matrix4<f32> {
        self.inverse_projection
    }

    // TODO: this seems to be too lenient
    /// Determines whether a point falls outside the viewing frustum.
    pub fn cull_point(&self, p: Vector3<f32>) -> bool {
        for plane in self.clipping_planes.iter() {
            if (self.view_projection() * p.extend(1.0)).dot(*plane) < 0.0 {
                return true;
            }
        }

        false
    }
}

#[repr(C, align(256))]
#[derive(Copy, Clone)]
// TODO: derive Debug once const generics are stable
pub struct FrameUniforms {
    // TODO: pack frame values into a [Vector4<f32>; 16],
    lightmap_anim_frames:  [Vector4<f32>; 16], // [UniformArrayFloat; 64],
    camera_pos: Vector4<f32>,
    time: f32,

    // TODO: pack flags into a bit string
    r_lightmap: UniformBool,
}

#[repr(C, align(256))]
#[derive(Clone, Copy, Debug)]
pub struct EntityUniforms {
    /// Model-view-projection transform matrix
    transform: Matrix4<f32>,

    /// Model-only transform matrix
    model: Matrix4<f32>,
}

enum EntityRenderer {
    Alias(AliasRenderer),
    Brush(BrushRenderer),
    Sprite(SpriteRenderer),
    None,
}

/// Top-level renderer.
pub struct WorldRenderer {
    worldmodel_renderer: BrushRenderer,
    entity_renderers: Vec<EntityRenderer>,

    world_uniform_block: DynamicUniformBufferBlock<EntityUniforms>,
    entity_uniform_blocks: RefCell<Vec<DynamicUniformBufferBlock<EntityUniforms>>>,
}

impl WorldRenderer {
    pub fn new(state: &GraphicsState, models: &[Model], worldmodel_id: usize) -> WorldRenderer {
        let mut worldmodel_renderer = None;
        let mut entity_renderers = Vec::new();

        let world_uniform_block = state.entity_uniform_buffer_mut().allocate(EntityUniforms {
            transform: Matrix4::identity(),
            model: Matrix4::identity(),
        });


      
        for (i, model) in models.iter().enumerate() {

            println!("assigning a modelkind for an entity {}, {}, {}",  i ,  model.name() ,  model.kind() ) ;
            println!("worldmodel id is {}", worldmodel_id);

            if i == worldmodel_id {
                match *model.kind() {
                    ModelKind::Brush(ref bmodel) => {

                        println!("Building world renderer {}", model.name());
                        worldmodel_renderer = Some(
                            BrushRendererBuilder::new(bmodel, true)
                                .build(state)
                                .unwrap(),
                        );
                    }
                    _ => panic!("Invalid worldmodel"),
                }
            } else {
                match *model.kind() {
                    ModelKind::Alias(ref amodel) => entity_renderers.push(EntityRenderer::Alias(
                        AliasRenderer::new(state, amodel).unwrap(),
                    )),

                    ModelKind::Brush(ref bmodel) => {
                        entity_renderers.push(EntityRenderer::Brush(
                            BrushRendererBuilder::new(bmodel, false)
                                .build(state)
                                .unwrap(),
                        ));
                    }

                    ModelKind::Sprite(ref smodel) => {
                        entity_renderers
                            .push(EntityRenderer::Sprite(SpriteRenderer::new(&state, smodel)));
                    }

                    _ => {
                        warn!("Non-brush renderers not implemented!");
                        entity_renderers.push(EntityRenderer::None);
                    }
                }
            }
        }

        WorldRenderer {
            worldmodel_renderer: worldmodel_renderer.unwrap(),
            entity_renderers,
            world_uniform_block,
            entity_uniform_blocks: RefCell::new(Vec::new()),
        }
    }

    pub fn update_uniform_buffers<'a, I>(
        &self,
        state: &GraphicsState,
        camera: &Camera,
        time: Duration,

        entities: QueryIter<&PhysicsComponent, ()>,


        lightstyle_values: &[f32],
        cvars: &CvarRegistry,
    ) //where
        //I:  Iterator<Item = &'a ClientUnit>,
    {


           /*  for    phys_comp in ecs_iter {

              println!("A phys comp is here ");
         } */


        trace!("Updating frame uniform buffer");
        state
            .queue()
            .write_buffer(state.frame_uniform_buffer(), 0, unsafe {
                any_as_bytes(&FrameUniforms {
                    lightmap_anim_frames: {
                        let mut frames = [Vector4::new(0.0,0.0,0.0,0.0); 16];
                        for i in 0..16 {
                            let x = lightstyle_values[i*4+0];
                            let y = lightstyle_values[i*4+1];
                            let z = lightstyle_values[i*4+2];
                            let w = lightstyle_values[i*4+3];

                            frames[i] = Vector4::new(x,y,z,w);
                        }
                        frames
                    }, 
                    camera_pos: camera.origin.extend(1.0),
                    time: engine::duration_to_f32(time),
                    r_lightmap: UniformBool::new(cvars.get_value("r_lightmap").unwrap() != 0.0),
                })
            });

        trace!("Updating entity uniform buffer");
        let world_uniforms = EntityUniforms {
            transform: camera.view_projection(),
            model: Matrix4::identity(),
        };
        state
            .entity_uniform_buffer_mut()
            .write_block(&self.world_uniform_block, world_uniforms);

        for (ent_pos, ent) in entities.into_iter().enumerate() {


            let ent_uniforms = EntityUniforms {
                transform: self.calculate_mvp_transform(camera, unit_origin,unit_angles,unit_model_id),
                model: self.calculate_model_transform(camera, unit_origin,unit_angles,unit_model_id),
            };

            if ent_pos >= self.entity_uniform_blocks.borrow().len() {
                // if we don't have enough blocks, get a new one
                let block = state.entity_uniform_buffer_mut().allocate(ent_uniforms);
                self.entity_uniform_blocks.borrow_mut().push(block);
            } else {
                state
                    .entity_uniform_buffer_mut()
                    .write_block(&self.entity_uniform_blocks.borrow()[ent_pos], ent_uniforms);
            }
        }

        state.entity_uniform_buffer().flush(state.queue());
    }

    

    pub fn render_pass_using_ecs<'a, E, P>(){


        
    }


    //w is world 
    // s is gamestate 

    //consider less generics here! kind of insane haha 

    pub fn render_pass<'a,    >(
        &'a self,
        state: &'a GraphicsState,
        pass: &mut wgpu::RenderPass<'a>,
        bump: &'a Bump,
        camera: &Camera,
        time: Duration,
   
     //   entitiesIteratorLegacy: E,
     //   particles: P,
        lightstyle_values: &[f32],   //should be an ecs resource ? 
        viewmodel_id: usize,
        cvars: &CvarRegistry,
        ecs_world:  &mut BevyWorld, //not ideal -- is there  a better way to pass less in here ?
    ) //where
       
        
       // E: Iterator<Item = &'a ClientUnit> + Clone,
       // P: Iterator<Item = &'a Particle>,
    {   
        //why must ecs world be mut to query !? 
        let mut query =  ecs_world.query::<  &PhysicsComponent  >();
        let phys_comp_iter = query.iter( ecs_world ) ;
        
        
       //let lightstyle_values = ecs_world.get_resource(); 

    
        

        use PushConstantUpdate::*;
        info!("Updating uniform buffers");
        self.update_uniform_buffers(
            state,
            camera,
            time,
            phys_comp_iter ,//entitiesIteratorLegacy.clone(),
            lightstyle_values,
            cvars,
        );

        pass.set_bind_group(
            BindGroupLayoutId::PerFrame as u32,
            &state.world_bind_groups()[BindGroupLayoutId::PerFrame as usize],
            &[],
        );

        // draw world
        info!("Drawing world");
        pass.set_pipeline(state.brush_pipeline().pipeline());
        BrushPipeline::set_push_constants(
            pass,
            Update(bump.alloc(brush::VertexPushConstants {
                transform: camera.view_projection(), //is this busted
              //  model_view: camera.view(),
            })),
            Clear,
            Clear,
        );
        pass.set_bind_group(
            BindGroupLayoutId::PerEntity as u32,
            &state.world_bind_groups()[BindGroupLayoutId::PerEntity as usize],
            &[self.world_uniform_block.offset()],
        );
        self.worldmodel_renderer
            .record_draw(state, pass, &bump, time, camera, 0);

        // draw entities
        info!("Drawing entities");
        for (ent_pos, ent) in entitiesIteratorLegacy.enumerate() {
            pass.set_bind_group(
                BindGroupLayoutId::PerEntity as u32,
                &state.world_bind_groups()[BindGroupLayoutId::PerEntity as usize],
                &[self.entity_uniform_blocks.borrow()[ent_pos].offset()],
            );

            match self.renderer_for_entity(unit_model_id) {
                EntityRenderer::Brush(ref bmodel) => {
                    pass.set_pipeline(state.brush_pipeline().pipeline());
                    BrushPipeline::set_push_constants(
                        pass,
                        Update(bump.alloc(brush::VertexPushConstants {
                            transform: self.calculate_mvp_transform(camera, unit_origin,unit_angles,unit_model_id),
                           // model_view: self.calculate_mv_transform(camera, ent),
                        })),
                        Clear,
                        Clear,
                    );
                    bmodel.record_draw(state, pass, &bump, time, camera, ent.frame_id);
                }
                EntityRenderer::Alias(ref alias) => {
                    pass.set_pipeline(state.alias_pipeline().pipeline());
                    AliasPipeline::set_push_constants(
                        pass,
                        Update(bump.alloc(alias::VertexPushConstants {
                            transform: self.calculate_mvp_transform(camera, unit_origin,unit_angles,unit_model_id),
                            model_view: self.calculate_mv_transform(camera, unit_origin,unit_angles,unit_model_id),
                        })),
                        Clear,
                        Clear,
                    );
                    alias.record_draw(state, pass, time, ent.frame_id(), ent.skin_id());
                }
                EntityRenderer::Sprite(ref sprite) => {
                    pass.set_pipeline(state.sprite_pipeline().pipeline());
                    SpritePipeline::set_push_constants(pass, Clear, Clear, Clear);
                    sprite.record_draw(state, pass, ent.frame_id(), time);
                }
                _ => unimplemented!("non-brush renderers not implemented!"),   //trying to render something weird for an entity 
                // _ => unimplemented!(),
            }
        }

        let viewmodel_orig = camera.origin();
        let cam_angles = camera.angles();
        let viewmodel_mat = Matrix4::from_translation(Vector3::new(
            -viewmodel_orig.y,
            viewmodel_orig.z,
            -viewmodel_orig.x,
        )) * Matrix4::from_angle_y(cam_angles.yaw)
            * Matrix4::from_angle_x(-cam_angles.pitch)
            * Matrix4::from_angle_z(cam_angles.roll);

 

         // this is rendering the model of the weapon you are holding 

        match self.entity_renderers[viewmodel_id] {
            EntityRenderer::Alias(ref alias) => {
                pass.set_pipeline(state.alias_pipeline().pipeline());
                AliasPipeline::set_push_constants(
                    pass,
                    Update(bump.alloc(alias::VertexPushConstants {
                        transform: camera.view_projection() * viewmodel_mat,
                        model_view: camera.view() * viewmodel_mat,
                    })),
                    Clear,
                    Clear,
                );
                alias.record_draw(state, pass, time, 0, 0);
            }

            _ => warn!("non-alias viewmodel"),  //was unreachable 
        }

        log::debug!("Drawing particles");
        state
            .particle_pipeline()
            .record_draw(pass, &bump, camera, particles);
    }

    /*fn renderer_for_entity(&self, ent: &ClientUnit) -> &EntityRenderer {
        // subtract 1 from index because world entity isn't counted
        &self.entity_renderers[ent.model_id() - 1]
    }*/

    fn renderer_for_entity(&self, model_id: usize) -> &EntityRenderer {
        // subtract 1 from index because world entity isn't counted
        &self.entity_renderers[model_id - 1]
    }

    fn calculate_mvp_transform(&self, camera: &Camera, origin:Vector3<f32>, angles:Vector3<Deg<f32>>, model_id: usize) -> Matrix4<f32> {
        let model_transform = self.calculate_model_transform(camera, origin,angles,model_id);

        camera.view_projection() * model_transform
    }

    fn calculate_mv_transform(&self, camera: &Camera, origin:Vector3<f32>, angles:Vector3<Deg<f32>>, model_id: usize) -> Matrix4<f32> {
        let model_transform = self.calculate_model_transform(camera, origin,angles,model_id);

        camera.view() * model_transform
    }

    fn calculate_model_transform(&self, camera: &Camera, origin:Vector3<f32>, angles:Vector3<Deg<f32>>, model_id: usize) -> Matrix4<f32> {
        //let origin = entity.get_origin();
        //let angles = entity.get_angles();
        let rotation = match self.renderer_for_entity(model_id) {
            EntityRenderer::Sprite(ref sprite) => match sprite.kind() {
                // used for decals
                SpriteKind::Oriented => Matrix4::from(Euler::new(angles.z, -angles.x, angles.y)),

                _ => {
                    // keep sprite facing player, but preserve roll
                    let cam_angles = camera.angles();

                    Angles {
                        pitch: -cam_angles.pitch,
                        roll: angles.x,
                        yaw: -cam_angles.yaw,
                    }
                    .mat4_quake()
                }
            },

            _ => Matrix4::from(Euler::new(angles.x, angles.y, angles.z)),
        };

        Matrix4::from_translation(Vector3::new(-origin.y, origin.z, -origin.x)) * rotation
    }
}
