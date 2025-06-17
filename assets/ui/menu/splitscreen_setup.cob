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
        width: 100%
        height: 100%
    }
    "jaaj"
        TextLine{ text: "Hello, World!" }
