#defs

$weapon_icon = "img/weapons/assaultrifle.png"

#commands
LoadImages[$weapon_icon]

#scenes
"hud_container"
    FlexNode{
        flex_direction: Column
        justify_main:FlexEnd
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
                padding: {right: 10px left: 10px}
            }
            "a_weapon_preview"
                FlexNode{
                    width:160px
                    height: 80px
                    justify_cross: Center
                }
                "image"
                    FlexNode{
                        width: 100%
                    }
                    LoadedImageNode{ image: $weapon_icon }
            "b_bullets_count"
                TextLine{ text: "42/69" }
        "weapons_list"
            FlexNode{
                flex_direction: Row
                justify_main: SpaceEvenly
                width: 100%
                padding: {top: 3px bottom: 3px}
            }
            BackgroundColor(#60353535)
            BrRadiusBottomRight(8px)
            BrRadiusBottomLeft(8px)
            
"weapon_entry"
    TextLine{ text: "Rifle" size: 18 }
