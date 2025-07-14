#defs

$weapon_icon = "img/weapons/pistol.png"
$ammunition_icon = "img/icons/ammunition-icon.png"
$heart_icon = "img/icons/heart-icon.png"

#commands
LoadImages[$weapon_icon $ammunition_icon $heart_icon]

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
                justify_main: SpaceBetween
                justify_cross: Center
                column_gap: 30px
                width:300px
                padding: {right: 10px left: 10px}
            }
            "a_weapon_preview"
                FlexNode{
                    width:150px
                    height: 80px
                    justify_cross: Center
                    clipping: ClipXY
                }
                "image"
                    FlexNode{
                        width: 100%
                    }
                    LoadedImageNode{ image: $weapon_icon }
            "infos"
                FlexNode{
                    flex_direction: Column
                    padding: {right: 8px}
                }
                "bullets_count"
                    FlexNode{
                        width: 100%
                        flex_direction: Row
                        justify_main: FlexEnd
                    }
                    "text"
                        TextLine{ text: "42/69" }
                    "icon"
                        FlexNode{
                            height: 24px
                            margin: {left: 8px}
                        }
                        LoadedImageNode{ image: $ammunition_icon }
                "percentage"
                    FlexNode{
                        width: 100%
                        flex_direction: Row
                        justify_main: FlexEnd
                    }
                    "text"
                        TextLine{ text: "69%" }
                    "icon"
                        FlexNode{
                            height: 24px
                            margin: {left: 8px}
                        }
                        LoadedImageNode{ image: $heart_icon }
            "reload_bg"
                AbsoluteNode{
                    height: 100%
                }
                ZIndex(-1)
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
