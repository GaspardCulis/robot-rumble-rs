#defs

$gamepad_icon = "img/icons/gamepad-icon.png"

+animation = \
    Animated<Margin>{
        idle: { left: 0px }
        hover: { left: 10px }
        hover_with: {
            duration: 0.2,
            ease: OutExpo
        }
        unhover_with: {
            duration: 0.2,
            ease: OutExpo
        }
        press: { left: 10px }
        press_with: {
            duration: 0.2,
            ease: OutElastic
        }
    }
\

#commands
LoadImages[$gamepad_icon]

#scenes
"splitscreen_setup"
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
            TextLine{ text: "Waiting for controllers" }
    "container"
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
            TextLine{ text: "Start match" }

"player_info"
    FlexNode{
        flex_direction: Column
        justify_cross: Center
    }
    Splat<Padding>(8px)
    BackgroundColor(#668888ff)
    BrRadius(8px)

    "text"
        TextLine{text: "Player X"}
    "gamepad_icon"
        FlexNode{
            width: 100px
            height: 100px
            margin: { right: 10px left: 10px top: -20px }
        }
        LoadedImageNode{ image: $gamepad_icon }