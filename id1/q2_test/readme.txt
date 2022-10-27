  CONTENTS

  System Requirements
  Release Notes

I. The Basics of Play
  A. Goal of the Game
  B. Skill
  C. Moving Around
  D. Exploring the World
  E. Environmental Hazards
II. Controls
  A. Keyboard Commands
  B. Main Menu
  C. The Console
III. The Game
  A. The Screen
  B. Firepower
  C. Special Items
  D. The Enemy
  E. Ending A Level

FAQS
Known Bugs
Reporting Bugs


  SYSTEM REQUIREMENTS

In order to play with Quake II Test you will need at least the following
computer system:

P90 w/16mb of RAM (P133 recommended)
24mb of RAM required when running GL
40mb of HD space

Latest version of Glide (2.4x) required in order to use 3Dfx with this test.
The Glide drivers can be found at www.3dfx.com


  RELEASE NOTES

This demo is not intended to represent the final state of Quake 2, but to
help us uncover architectural problems. There will be another demo available
before the game hits store shelves that should be an accurate representation
of the final product.

Some of the significant issues with the current demo are:

User control.  Currently, the user input is only sampled at 10hz, which
means that you can miss fast button presses, and it will generally feel a
little sluggish. Movement underwater and on slopes is in a temporary state.
There are also some problems with the player getting stuck in certain
circumstances that need to be resolved.

Gameplay tuning.  Most of the work remaining to us is the fine tuning of
weapons and monsters for best effect.  The current state is not
representative of the final state.

Networking.  Multiplayer games are disabled in this test.  We have not
finished the rules or compressed the communication.

Optimization.  We have a few more data and code optimizations to implement
that will result in some speed ups.  Probably about 5% to 10% during actual
gameplay.

Data.  In the interest of keeping the demo download size reasonable, the
high quality sound files and cinematics are not included for testing.  The
map files have some known T junctions in them, which can cause single pixel
dots to show through in places until we fix that.

User extensibility.  This demo is explicitly NOT user extendable.  There is
still a little bit of cleanup to be done before we can release the game dll
source code, and the retail game will be required to use modified data.

Demo loading. When starting the game it may take a while for the demo to load.
During this time you might suspect that you machine has hung. This is probably
not the case.

I. THE BASICS OF PLAY

A. GOAL OF THE GAME
    The basic goal in Quake II is to stay alive and complete your missions.
    When entering a new level your personal computer (press F1) is updated
    with information about the tasks you must complete in order to progress
    through the game. You are presented with a mission objective and
    instructions on how to achieve your objective(s).

    You are also given counters for your KILLS, GOALS, and, SECRETS. These are
    displayed on the personal computer. Whenever new information is added an
    icon will appear at the bottom of the main screen.

    The overall goal for the Quake II test is to establish a communication
    uplink to your command ship. Your instructions on how to achieve this
    goal will change as you progress through the levels. 
    
B. SKILL
    Before starting the game you may select a difficulty level from the
    GAME menu.
    
	EASY:       This is for those of you who are slow and inept.
    MEDIUM:     For those of you who have it somewhat together.
    HARD:       This is how we think you should play the game.
    NIGHTMARE:  Designed to please the masochist in all of us.

   SKILL SETTINGS ARE NON-FUNCTIONAL IN THE QUAKE II TEST. THEY WILL BE
   AVAILABLE IN FUTURE RELEASES.

C. MOVING AROUND
        The keys listed below can be changed in the Customize Controls Menu.

    Walking
    Use the four arrow keys or the mouse. To walk steadily forward,
    hold down the Forward key (up arrow or left button on the mouse). Turn
    left or right with the left or right arrow keys, or by sliding your mouse
    in the desired direction.

    Running
    Hold down the left Shift key to increase your speed. If you
    prefer to always run during the game, open the Main Menu, then the
    Controls menu, and select "Always Run".

    Shooting
    Tap the CTRL key or the RIGHT mouse button to fire. Hold shoot
    down to keep firing.

    Jumping
    Tap the SPACE bar to jump. You jump farther if you're moving
    forward at the same time, and you jump higher if you're moving up a slope
    at the same time. You'll be surprised at the spots you can reach in a
    jump. You can even avoid some attacks this way.

    Ducking
    Press and hold down the "C" key to duck and crawl ("C" while
    moving forward).  When you release the "C" key, you will return to an
    upright position. It is also possible to avoid rockets in this manner.

    Climbing Ladders
    When you encounter a ladder press and hold down the jump key (SPACE bar)
    to climb. 

    Swimming
    While underwater, aim yourself in the direction you wish to go
    and press the forward key to go in that direction. Unfortunately, as in
    real life, you may lose your bearing while underwater. Use the jump key
    (SPACE bar) to kick straight up towards the surface. Once on the surface,
    tread water by holding down jump. To get out of the drink, swim toward the
    shore and you can either jump up onto the land or walk straight out if it
    is shallow enough.  There is always a way out but you may have to submerge
    even deeper in order to find it.  

    Looking Up and Down
    The letters "A" and "Z" allow you to look up and down, respectively.

    Strafing
    Hold down either "Alt" key while the left or right arrow key
    is pressed and you will side-step in that particular direction. This
    is perfect for avoiding incoming missiles, rockets or gun blasts from
    enemy Strogg.

    Picking Up Objects
    To pick up items, weapons, and power-ups, simply walk
    over them. If you are unable to pick something up, it means you already
    have the maximum possible for that object.

    Selecting Items in Inventory
    Hit the TAB key to access your inventory 
    display. Use the square "[ ]" bracket keys to cycle through items in your
    inventory. Press the Enter key to use a highlighted item. 

D. EXPLORING THE WORLD

    Buttons and Floorplates
    Buttons on walls or posts will activate with a
    touch, and floorplates must be stepped on. Use your discretion - if
    you see a distinctive-looking button in a spot you cannot reach, it's
    probably a shootable one - fire at it to activate.

    Doors
    As with most places on Earth, the majority of doors here open at
    your approach. If one doesn't, then seek a button, floorplate, or key.

    Secret Doors
    Some doors are camouflaged. Almost all secret doors open
    when they are shot, so keep an eye out for these. The rest are opened
    by hidden pressure plates, buttons, or levers.

    Platforms
    Most platforms only go up and down, while some follow
    tracks around rooms or levels (i.e. horizontally). Normally, when
    you step onto a platform it rises to its full height and lowers when
    you step off (and watch your head going up!). Some platforms drop when
    you step atop them, and some don't work until you activate them via a
    button, pressure plate, or shootable target.

    Pressure Plates and Motion Detectors
    These can be invisible or
    visible sensors which open doors, unleash traps, warn monsters,
    and so forth.

    Uncovering Secrets
    Secret areas on Stroggos are hidden in various
    ways (those aliens are sneaky MF'ers). In order to uncover these,
    you might need to shoot a button, kill a cyborg, walk through a secret
    motion detector, etc. All secrets are indicated by clues, so don't
    waste your time hacking on every wall or shooting every object in the
    room. Use your brain, your eyes, and your ears for something odd…then
    act on it.

E. ENVIRONMENTAL HAZARDS

    Explosions
      Harmful radioactive containers are scattered throughout some military
      bases so watch your gunfire near these. Also, make sure you're not
      leaning up against one during a bullet exchange with the enemy or you
      may end up in 46 little body bags. Keen Marines know when to use these
      explosives to their advantage - aim for one when aliens are around them,
      or a radioactive container may blow open a hole in the wall or floor
      as an escape route.

    Water
      The water on Stroggos is safe enough to enter without needing a
      bio-suit, but remember to come up for air periodically for a deep
      breath.

    Slime
     Toxic waste will harm you instantly and keeps on tearing at your flesh
     the longer you stay submerged. Unless you have a bio-suit, stay clear
     from the slime.

    Traps
      The Strogg may be ugly as sin, but they are not a stupid race. Their
      planet is full of traps, so be on the look-out for laser shooters,
      aliens in ambush, trapdoors, and so forth. Don't be paranoid - just
      be aware of their existence.

II. CONTROLS
A. KEYBOARD COMMANDS
    You can use the key configuration option from the Controls Menu
    (Press F4) to customize the keyboard however you like, except for
    the Function Keys, Escape Key, and the ~ (tidle) Key.

FUNCTION KEYS
Personal Computer            F1
Save Game                    F2
Load Game                    F3
Key Config Menu              F4
Multiplayer Menu             F5
Quick Save                   F6
Quick Load                   F9
Quit To Operating System     F10
Screenshot                   F12

WEAPONS
Blaster                      1
Shotgun                      2
Super Shotgun                3
Machine Gun                  4
Chaingun                     5   NOT IN DEMO
Grenade Launcher             6
Rocket Launcher              7
Hyper Blaster                8   NOT IN DEMO
Rail Gun                     9   NOT IN DEMO
BFG                          0   NOT IN DEMO

ITEMS
Hand Grenade                 g
Quad Damage                  q
Rebreather                   b
Silencer                     s   NOT IN DEMO
Environment Suit             e
Invulnerability              i   NOT IN DEMO

MOVEMENT
Move / Turn                  Arrow Keys
Jump / Swim                  Space Bar
Run                          SHIFT
Strafe Left                  , or <
Strafe Right                 . or >
Strafe *                     ALT
Crouch                       C

OTHER CONTROLS
Inventory                    TAB
Main Menu                    ESC
Console                      ~ (tilde)
Look Up                      A or Page Up
Look Down                    Z or Page Down

B. THE MAIN MENU
    Hit the ESC key to bring up the Main Menu. While in the menu, the game
    is paused. Use the arrow keys to move the Quake II icon up and down the
    menu. Place the icon at the desired option and hit the Enter Key. To
    return to a previous menu, hit the ESC key again. To exit back to
    the game from the Main Menu, hit ESC.

    GAME
            Allows you to set difficulty, start a new game, load a previously
            saved game, save the current game, and start or join a network
            game.

    DIFFICULTY
            Allows you to choose EASY, MEDIUM, HARD or NIGHTMARE skill level.
            THIS FEATURE IS DISABLED IN THE DEMO.

    START GAME
            Starts a new game on the first level.

    LOAD
            This will bring up a list of currently saved game. Highlight
            the game you wish to play and hit the ENTER key.

    SAVE
            This will bring up a list of currently saved games. Highlight a
            slot and hit the ENTER key. Your game will be saved into this
            slot. You cannot save your game if you are dead.

    NETWORK OPTIONS ARE DISABLED IN THE QUAKE II TEST. THEY WILL BE AVAILABLE
    IN FUTURE RELEASES.
    
    VIDEO

        DRIVER
            The video menu currently allows you to select one of four
            rendering subsystems: software, system OpenGL, 3Dfx OpenGL,
            and PowerVR OpenGL. The software driver is available on all
            systems. The system OpenGL driver allows Quake2 to render
            scenes using the default OpenGL driver installed in	the system.
            Typically this will be selected under Windows NT when using a
            2D/3D accelerator such as an Intergraph Realizm or accelerators
            based on the Nvidia RIVA128, ATI Rage Pro, and Rendition V2200.
            It is not recommended to use the OpenGL subsystem on systems
            that do not have hardware acceleration of OpenGL installed.
            The 3Dfx OpenGL driver should be used on systems that possess a
            3Dfx Voodoo and Voodoo Rush accelerator, including the Canopus
            Pure3D, Diamond Monster3D, Orchid Righteous 3D, and the Hercules
            Stingray 128.  The PowerVR OpenGL subsystem should be used on
            those systems that are running Win95 with a PowerVR PCX2 board
            installed, such as the Matrox M3D.

            Future versions of Quake2 may support other rendering subsystems.
            At this time Quake2 does not support the Microsoft Direct3D
            proprietary API.

        VIDEO MODE
            Quake2 supports the following video modes:

            * 320x240
            * 400x300
            * 512x384
            * 640x480
            * 800x600
            * 960x720
            * 1024x768
            * 1152x864
            * 1280x960
            * 1600x1200

            Availability of video modes will be limited by the type of
            graphics adapter installed and available system and video RAM.
            For example, boards based on the 3Dfx Voodoo chipset typically
            only support video modes of 512x384 and 640x480.

        SCREEN SIZE
            The screen size slider controls the size of the visible area on
            the screen.  Reducing the screen size will usually result in
            higher performance.

        BRIGHTNESS
            The brightness slider controls the brightness of the screen.  Its
            effects are immediate in the software driver, but do not take
            effect until "Apply" is selected when using any of the OpenGL
            subsystems.

        FULLSCREEN
           This selects fullscreen vs. windowed rendering.  On 3Dfx Voodoo
           graphics rendering to a window is not supported.  Fullscreen mode
           availability is dependent upon the type of graphics adapter
           installed.  On many graphics adapters, modes such as 400x300 and
           512x384 are not available with fullscreen rendering. Fullscreen
           software rendering requires the presence of Microsoft DirectX.
           With the OpenGL subsystem fullscreen rendering will use whatever
           color depth the desktop is currently set to.

        TEXTURE QUALITY (OpenGL ONLY)
           The texture quality slider determines the overall crispness of
           textures with OpenGL renderers.  Better quality often results
           in lower performance.

        8-Bit TEXTURES (Open GL ONLY)
           Support for 8-bit paletted textures is available on some graphics
           chipsets such as the 3Dfx Voodoo. Enabling 8-bit textures results
           in a slight loss of visual quality in exchange for better overall
           performance.

        STIPPLE ALPHA (SOFTWARE ONLY)
            Enabling stipple alpha results in faster performance when
            rendering transparent surfaces such as windows, water, and
            lava, but also results in reduced image quality when rendering
            transparent surfaces.

        APPLY
           Selecting this option "applies" the given video configuration,
           which means that the selected options are made current if possible.

      A NOTE ABOUT VIDEO OPTIONS
        Some video options available in Quake II are not supported by your
        particular hardware. If you select an unsupported video option Quake
        II will return to the previous video settings or a default software
        mode and drop down the console to report the error.


     AUDIO
        This menu will allow you to adjust the volume of the in game
        sound effects, set the quality of the sound effects, and turn
        CD music on or off. THE QUAKE II DEMO DOES NOT INCLUDE HIGH
        QUALITY SOUNDS.

     CONTROLS
        Here you can adjust your mouse options, customize your keyboard
        settings, and restore all your control settings to their defaults.

     MOUSE SPEED
        Allows you to adjust your mouse movement speed. The higher you
        set this the faster your character will turn in relation to 
        mouse movement.

     ALWAYS RUN
        You can set this to YES if you do not want to hold down the RUN
        button in order to move quickly.

     INVERT MOUSE
        This gives your mouse "airplane-style" controls.  This means 
        that pushing the mouse forward "noses down", and pulling it 
        back "noses up".  Some people prefer this control technique.

     LOOKSPRING
        Returns your view immediately to straight ahead when you 
        release the look up / down key.  Otherwise, you must move 
        forward for a step or two before your view snaps back.
        Lookspring does not work while you are underwater. 

     LOOKSTRAFE
        If you are using the look up / down key, then this option 
        causes you to sidestep instead of turn when you try to move 
        left or right.

      FREELOOK
        With this option enabled you no longer have to press the
		MLOOK key to look up and down while using the mouse.

C. CONSOLE
        Tap the ~ (tilde) key to bring down the console.  As with the Main
        Menu, when the console is down, a single player game is paused.  A wide
        variety of esoteric commands can be entered at the console.

III. THE GAME
A. THE SCREEN
     The large top part of the screen is the view area, in which you see
     monsters and architecture. You can enlarge the viewing area by hitting
     the + key. The - key shrinks the view area.
    
    Health Display
        This displays the amount of life you have remaining. It also shows
        a picture of your face which can give you a rough idea of how much
        damage you have sustained. Your face can also indicate the direction
        from which damage is being inflicted.

    Ammo Display
        The type and amount of ammo remaining is displayed here.

    Armor Display
        The amount of armor you have remaining is displayed here. The armor
        type is represented by an icon to the side of this Display. If you are
        completely out of armor this display will disappear.

    Selected Item Display
        This displays the item that is ready to be used. Use the bracket keys
        to move through the list and press the ENTER key to activate it.
    
    Active Power Up Display
        If you select a power up that has a duration Quake II will display it
        on the screen with a count down to show the time remaining.

    Personal Computer
        When your personal computer receives new information an icon will
        appear at the bottom center of the game screen. Press the F1 key to
        pull up your personal computer. It will display your current mission
        objectives, secondary objective and your current kills, secrets, and
        objectives solved. Some levels will require you to leave and then
        re-enter in order to attain all kills, secrets, and goals.

    Inventory
       Press the tab key to pull down your inventory screen. Use the bracket
       keys to navigate through the list and press the ENTER key to active
       the currently selected item. The inventory displays the item name, the
       hot key for using it, and the amounts you have of each item.

B. FIREPOWER

      Blaster
        The common rechargeable energy side-arm as used on Earth. As you know,
        the Blaster is not much, but it's good enough to gore a guard or
        tickle a tank. 

      Shotgun
        This mighty sprayer carries a lot of ammo but is virtually useless at
        a distance. If possible, get in nice and close to use the Shotgun
        instead of your Blaster to add some lead freckles on the Strogg's ugly
        faces. Remember, it requires shells and takes some time to reload.

      Super Shotgun
        This is the uncompromising big brother to the Shotgun, intended for
        a close encounter of the dangerous kind. It eats more shells than the
        Shotgun, but the show is well worth the price of admission.

      Machine Gun
        Perfect for putting guards in their place - in hell. Although this
        effective weapon is easy to use, its light weight causes considerable
        kickback and it may force your gun up, ruining your aim.

      Grenade Launcher
        Fast, easy-to-aim explosives designed for long-range use in mind.
        Not recommended for smaller, confined areas or you may also blow 
        up 'real good'.

      Hand Grenade
        Just twist open the cap and toss it. Treat this like a really
        dangerous baseball, and lob it in the direction you choose. Just
        remember to let go at some point. The longer you hold down the
        fire key, the further you will toss the grenade.

      Rocket Launcher
        A slow but deliciously lethal weapon of choice. Period.

C. SPECIAL ITEMS

      Ammo
        There are four major ammo types: shells, bullets, grenades, and
        rockets. With the exception of the laser blaster, you must have
        ammunition to use a weapon. Each ammo type has a maximum you can
        carry.

      Armor
        There are three armor types: Flak Jacket, Combat Suit, and Body Armor.
        Each one provides a certain amount of protection against both normal
        attacks and energy weapon attacks. Your current armor type will be
        displayed on your status bar. If you take enough hits, your armor 
        strength will deplete down to nothing, so seek out unused breast
        plates. If you pick up armor that is as good as or worse than your
        current armor, it is salvaged to improve your current armor level.

      Armor Shards
        Special remnant of armors, which add a bit more durability to your
        existing protection.

      Health
        There are two types of standard health kits to ingest: First Aid
        and Medkits.

      Stimpacks
        These provide an additional boost to your health.

      Bandolier
        Also increases potential inventory for each ammo type, except for
        explosives such as grenades and rockets.

      Underwater Breather
        Provides oxygen when submerged in liquids.

      Quad Damage
        Temporarily multiplies all your weapon's strengths by four times.
        Let the gibbing begin!

      Adrenaline Pack
        This adds an additional two health to your overall achievable health
        level.

D. THE ENEMY

    Name: Light Guard
    Description: Weakest of the three processed humans, armed solely with
    a simple blaster.

    Name: Shotgun Guard
    Description: These loyal troops are equipped with an automatic scatter gun
    prosthetic.

    Name: Machine-gun Guard
    Description: Bigger, meaner, and deadlier than above…with a machine-gun
    for a right arm.

    Name: Enforcer
    Description: Strong, muscle-bound warrior who dishes out chain gun speed
    damage. If he gets close enough he will try to whack you over the head 
    with his gun.

    Name: Gunner
    Description: The fighting elite for the Strogg, outfitted with a powerful
    machine gun and an automatic grenade launcher.

    Name: Berserker
    Description: He has a metal spike as one arm, a hammer as another, plus
    two crazy-fast legs.

    Name: Parasite
    Description: Four-legged beast with a weapon on its back. Once fired, it
    attaches itself and literally sucks the life from you.

    Name: Flyer
    Description: A small two-winged monster, comprised of a controlling brain
    and a cyborg body that allows it to levitate.

    Name: Tank
    Description: Tanks are armed with three weapons that they use at random:
    an arm-mounted machine gun, an arm-mounted laser blaster, a
    shoulder-mounted rocket launcher.

E. ENDING A LEVEL

     Once you finish a level, you'll find an elevator or passageway that takes
     you to the next level. You start the next level with the same armor,
     ammo, health, and weapons you had at the end of the previous one. If you
     were using a power-up as you ended the previous level it will now be
     deactivated. In some cases you can return to a previous level.

FAQs

Q. I'm stuck. How do I get through the level?
A. Take a stroll around and look for a place you haven't been yet.  Sometimes
you have to kill a particular monster in order to progress, so exterminate 
them all!

Q. How can I find all the secrets? 
A. Don't worry about it.  You never have to find a secret to finish a level.
Also, some secrets are intentionally hard to find. 

Q. I've cleared out the whole level, but my monster kill score isn't 100%. 
Where are they hiding?
A. Some monsters hide inside secrets, or are released by them.  You won't be 
able to kill those monsters until you find their secrets. You also may have
to leave the level and then return to release all the monsters.

Q. Did I really see two monsters fighting each other?
A. Probably.  Some monsters hate one another almost as much as they hate you. 
You can use this to your advantage (exercise left up to the reader). 

KNOWN BUGS

Occasionally your character may become stuck in a wall. Go to the console by
hitting the ~ (tilde) key and type NOCLIP. Move away from the wall and type
NOCLIP again.

Monster AI is still in development. Some monsters may seem to be ignoring you
until you actually shoot them. This is because as small aliens they refused
to eat their humans and as a result suffered brain damage. We will be taking
measures to ensure that all aliens receive a proper diet before the final 
release.

Door and plat sounds may malfunction occasionally.

Some areas may cause you to get a "packet overflow" message.

Auto-restart load games sometimes load you one level back from the level
you should be started on.

A quick saved game can only be loaded from the quick load menu.
A normal saved game can only be loaded from the normal load game menu.

The in-game demo will only play one time and then end in an error.

REPORTING BUGS

Bug Reporting for Quake II Compatibility Test:

We have prepared a form to make submitting bugs easy for the player.  This
will ensure that we get the information we need to accurately identify and
fix these problems.

To submit a bug report, just point your web browser to:
http://www.idsoftware.com/contact/q2feedback.html or
http://www.activision.com/games/action/quake2/q2feedback.html or
http://www.stomped.com/q2feedback.html or
http://www.bluesnews.com/ or
http://www.gamecenter.com/Quake2/q2feedback.html

Once there, all you have to do is fill in the blanks and hit the button at
the bottom of the page to submit your report.  Please be as thorough as
possible in filling out the form.  Also, please note that someone may contact
you for more information if needed.

Thanks for your help!

id Software
