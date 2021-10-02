module Main exposing (..)

-- import Html exposing (Html, audio, button, div, h1, img, text)
-- import Html.Attributes exposing (id, src, style)
-- import Html.Events exposing (onClick)

import Browser
import Html exposing (Html, div, h1, img, text)
import Html.Attributes exposing (src)
import Http
import Json.Decode exposing (Decoder, field, string)



---- MODEL ----


type alias User =
    { name : String
    }


type alias Model =
    { name : Maybe String
    }


init : ( Model, Cmd Msg )
init =
    ( { name = Nothing }, getUser )



---- UPDATE ----


type Msg
    = GotUser (Result Http.Error User)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotUser result ->
            case result of
                Ok user ->
                    ( { model | name = Just user.name }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div []
        [ img [ src "/logo.svg" ] []
        , h1 []
            [ text
                ("My name is "
                    ++ (case model.name of
                            Just name ->
                                name

                            Nothing ->
                                "..."
                       )
                )
            ]
        ]



---- HTTP ----


getUser : Cmd Msg
getUser =
    Http.request
        { method = "GET"
        , headers = []
        , url = "http://localhost/api"
        , expect = Http.expectJson GotUser userDecoder
        , body = Http.emptyBody
        , timeout = Nothing
        , tracker = Nothing
        }


userDecoder : Decoder User
userDecoder =
    Json.Decode.map User
        (field "name" string)



---- PROGRAM ----


main : Program () Model Msg
main =
    Browser.element
        { view = view
        , init = \_ -> init
        , update = update
        , subscriptions = always Sub.none
        }
