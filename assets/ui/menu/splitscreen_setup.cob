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

"player_info"
    FlexNode{
        flex_direction: Column
        justify_cross: Center
    }
    BackgroundColor(Hsla{hue:138 saturation:0.23 lightness:0.57 alpha:1})
    BorderColor(Hsla{hue:174 saturation:0.23 lightness:0.18 alpha:1})
    BrRadius(6px)

    "text"
        TextLine{text: "Player X"}
    "gamepad_icon"
        FlexNode{
            width: 100px
            height: 100px
            margin: { right: 10px left: 10px top: -20px }
        }
        LoadedImageNode{ image: $gamepad_icon }