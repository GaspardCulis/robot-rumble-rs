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
"home"
    FlexNode{
        flex_direction: Column
        justify_main:Center
        width: 100%
        height: 100%
    }
    Splat<Padding>(3%)
    "multiplayer"
        +animation{}
        TextLine{text:"Multiplayer"}
        Splat<Padding>(8px)

    "local"
        +animation{}
        TextLine{text:"Local Play"}
        Splat<Padding>(8px)

    "settings"
        +animation{}
        TextLine{text:"Settings"}
        Splat<Padding>(8px)

    "credits"
        +animation{}
        TextLine{text:"Credits"}
        Splat<Padding>(8px)

    "quit"
        +animation{}
        TextLine{text:"Quit"}
        Splat<Padding>(8px)
    
    "background"
        AbsoluteNode{
            left: 0px
            top: 0px
            min_width: 0%
        }
        ZIndex(-1)
        LoadedImageNode{ /* Image set in code */ }