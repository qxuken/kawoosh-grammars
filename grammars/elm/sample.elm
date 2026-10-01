module Sample exposing (Shape(..), area, main)

import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)


{-| A sample.
-}
type Shape
    = Circle Float
    | Rect Float Float
    | Empty


type alias Model =
    { count : Int
    , shapes : List Shape
    }


type Msg
    = Increment
    | Reset


area : Shape -> Float
area shape =
    case shape of
        Circle r ->
            pi * r * r

        Rect w h ->
            if w > 0 then
                w * h

            else
                0

        Empty ->
            0


update : Msg -> Model -> Model
update msg model =
    case msg of
        Increment ->
            { model | count = model.count + 1 }

        Reset ->
            { model | count = 0 }


view : Model -> Html Msg
view model =
    let
        total =
            model.shapes |> List.map area |> List.sum
    in
    div []
        [ button [ onClick Increment ] [ text "+" ]
        , text (String.fromInt model.count ++ " " ++ String.fromFloat total)
        ]


main : Html Msg
main =
    -- A comment.
    view { count = 0, shapes = [ Circle 1, Rect 2 3, Empty ] }
