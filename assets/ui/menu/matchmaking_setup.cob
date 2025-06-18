#defs
$animation_settings_medium = {duration:0.1 ease:Linear}
$animation_settings_fast = {duration:0.1 ease:Linear}

#scenes
"matchmaking_setup"
    FlexNode{
        flex_direction: Column
        justify_main:Center
        justify_cross:Center
        width: 100%
        height: 100%
    }

    "header"
        FlexNode{
            flex_direction:Row
            justify_main:Center
            justify_cross:Center
            column_gap: 16px
        }
        "text"
            TextLine{ text: "Gamemode" }

    "container"
        RadioGroup
        FlexNode{
            flex_direction: Row
            justify_main:Center
            justify_cross:Center
            column_gap: 16px
            margin: { top: 80px }
        }

    "start_button"
        AbsoluteNode{
            top:auto left:auto
            right: 10px
            bottom: 10px
        }
        Splat<Padding>(20px)
        Responsive<BackgroundColor>{idle:#00000000 hover:#66888888 press:#668888ff}
        BrRadius(8px)
        "text"
            TextLine{ text: "Find match" }
    "back_button"
        AbsoluteNode{
            top:auto
            left: 10px
            bottom: 10px
        }
        Splat<Padding>(20px)
        Responsive<BackgroundColor>{idle:#00000000 hover:#66888888 press:#668888ff}
        BrRadius(8px)
        "text"
            TextLine{ text: "Back" }

"gamemode"
    RadioButton
    FlexNode{
        height: 100px
        justify_main:Center
        justify_cross:Center
        padding: {right: 10px left: 10px}
    }
    BrRadius(8px)
    "text"
        TextLine{ text: "XP match" }
    Multi<Animated<BackgroundColor>>[
        {
            idle: #668888ff
            hover: #446666ff
            hover_with: $animation_settings_medium
            press_with: $animation_settings_medium
        }
        {
            state: [Selected]
            idle: #446666ff
            hover: #446666ff
            enter_idle_with: $animation_settings_medium
            hover_with: $animation_settings_medium
            press_with: $animation_settings_medium
        }
    ]