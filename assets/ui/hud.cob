#defs

$weapon_icon = "img/weapons/assaultrifle.png"

#commands
LoadImages[$weapon_icon]

#scenes
"hud_container"
    FlexNode{
        flex_direction:RowReverse
        justify_main:SpaceBetween
        justify_cross:FlexEnd
        width: 100%
        height: 100%
    }
    Splat<Padding>(10px)
"hud"
    BackgroundColor(#40f5f5f5)
    BrRadius(8px)
    "vbox"
        FlexNode{
            flex_direction: Column
            justify_cross: Center
        }
        "hbox"
            FlexNode{
                flex_direction: Row
                justify_cross: Center
                column_gap: 30px
                width:300px
                padding: {right: 10px left: 10px}
            }
            "a_weapon_preview"
                FlexNode{
                    width:160px
                    height: 80px
                    justify_cross: Center
                    clipping: ClipXY
                }
                "image"
                    FlexNode{
                        width: 100%
                    }
                    LoadedImageNode{ image: $weapon_icon }
            "bullets_count"
                TextLine{ text: "42/69" }
            "reload_bg"
                AbsoluteNode{
                    width: 0%
                    height: 100%
                }
                BrRadiusTopRight(8px)
                BrRadiusTopLeft(8px)
                BackgroundColor(#80353535)
        "weapons_list"
            FlexNode{
                flex_direction: Row
                justify_main: SpaceEvenly
                width: 100%
            }
            BackgroundColor(#60353535)
            BrRadiusBottomRight(8px)
            BrRadiusBottomLeft(8px)
            
"weapon_entry"
    FlexGrow(1)
    Padding{top: 3px bottom: 3px}
    BackgroundColor(#aa353535)
    BrRadiusBottomRight(8px)
    BrRadiusBottomLeft(8px)
    "text"
        TextLine{ text: "Rifle" size: 18 justify: Center }
        Splat<Margin>(auto)
