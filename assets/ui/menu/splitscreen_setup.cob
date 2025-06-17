#defs
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

#scenes
"splitscreen_setup"
    FlexNode{
        flex_direction: Column
        justify_main:Center
        justify_cross:Center
        width: 100%
        height: 100%
    }
    "title"
        TextLine{ text: "Player select" }
    "container"
        FlexNode{
            flex_direction: Row
            justify_main:Center
            justify_cross:Center
            margin: { top: 80px }
        }

"player_config"
    FlexNode{
        flex_direction: Column
        justify_cross: Center
    }
    "text"
        TextLine{text: "Player 1"}
    "gamepad_icon"
        FlexNode{
            width: 100px
            height: 100px
            margin: { right: 10px left: 10px top: -20px }
        }
        LoadedImageNode{ /* Image set in code */ }