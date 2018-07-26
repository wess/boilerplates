module View exposing (root)

import Html             exposing (..)
import Html.Attributes  exposing (..)

import Models   exposing (Model)
import Messages exposing (Msg(..))
import Routing exposing (..)

linkView : String -> Html Msg
linkView name =
  a[class "nav-link", href ("#" ++ name)][text name]

menuView : Html Msg
menuView =
  div[class "container text-align-center"]
  [ a[class "nav-link", href ("")][text "home"]
  , linkView "about"
  , linkView "contact"
  ]

homeView : Html Msg
homeView = 
  div[class "container"]
  [ h1[][text "Home View"]
  ]

aboutView : Html Msg
aboutView = 
  div[class "container"]
  [ h1[][text "About View"]
  ]

contactView : Html Msg 
contactView = 
  div[class "container"]
  [ h1[][text "Contact View"]
  ]

notFoundView : Html Msg
notFoundView = 
  div[class "container"]
  [ h1[][text "404 Not Found View"]
  ]

page : Model -> Html Msg
page model =
  case model.route of
    Routing.Home ->
      homeView
    Routing.About ->
      aboutView
    Routing.Contact ->
      contactView
    Routing.NotFound ->
      notFoundView

content : Model -> Html Msg
content model = 
  div[id "main-content"]
  [ page model
  , menuView
  ]

root : Model -> Html Msg
root model =
    div[id "app"]
    [ content model
    ]


