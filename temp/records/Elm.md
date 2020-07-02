# Elm

## 介绍

* Elm是一种编译为JavaScript的功能性语言
* 功能性语言的特点
  * 无运行时错误
  * 有好的报错信息
  * 可靠的重构
  * 自动执行所有Elm包的语义版本控制

## 核心语言

### Values

* 包括：整形、浮点型、布尔值、字符串
* 可在页面上进行数字运算、字符串合并等等操作

### Functions

* 用来转化Values
* 相较于JS，Elm少了很多括号、逗号的使用，更加简洁

### IF语句

### Lists

* 具有相同类型的值的组合

### Tuples

* 可以具有2-3个values，每个value类型均可不同

### Records

* 一个Records可以拥有多个Values，且每个value会和一个name相关联
* 更新Records实际上是新建了一个Records，它并不会覆盖之前的，且他们共享不变的值

## The Elm Architecture

* 用以开发交互式项目

### 基本类型

![Diagram of The Elm Architecture](https://guide.elm-lang.org/architecture/buttons.svg)

* 如上，Elm产生HTML显示在屏幕上，而网页发送Msg给Elm
* Elm program包含以下三部分，也是Elm Architecture的核心
  * Model：应用的当前状态
  * View：把状态转化为HTML
  * Update：根据Msg更新状态

### Button

```elm
import Browser
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL

type alias Model = Int

init : Model
init =
  0


-- UPDATE

type Msg = Increment | Decrement

update : Msg -> Model -> Model
update msg model =
  case msg of
    Increment ->
      model + 1

    Decrement ->
      model - 1


-- VIEW

view : Model -> Html Msg
view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (String.fromInt model) ]
    , button [ onClick Increment ] [ text "+" ]
    ]
```

#### Main

* 决定什么会显示在屏幕上

#### Model

* 捕捉应用中所有的数据

#### View

* 以Model作为参数，产生HTML
* `onClick`：当有点击时，生成Msg

#### Update

* 描述Model的变化
* 当收到信息后，通过update可得到一个新的Model

#### 总过程

先初始化，然后执行如下循环

1. Wait for user input.
2. Send a message to `update`
3. Produce a new `Model`
4. Call `view` to get new HTML
5. Show the new HTML on screen
6. Repeat

### Text Fields

```elm
import Browser
import Html exposing (Html, Attribute, div, input, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
  { content : String
  }


init : Model
init =
  { content = "" }



-- UPDATE


type Msg
  = Change String


update : Msg -> Model -> Model
update msg model =
  case msg of
    Change newContent ->
      { model | content = newContent }



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ input [ placeholder "Text to reverse", value model.content, onInput Change ] []
    , div [] [ text (String.reverse model.content) ]
    ]
```

#### Model

#### View

#### Update

### Forms

```elm
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
  { name : String
  , password : String
  , passwordAgain : String
  }


init : Model
init =
  Model "" "" ""



-- UPDATE


type Msg
  = Name String
  | Password String
  | PasswordAgain String


update : Msg -> Model -> Model
update msg model =
  case msg of
    Name name ->
      { model | name = name }

    Password password ->
      { model | password = password }

    PasswordAgain password ->
      { model | passwordAgain = password }



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ viewInput "text" "Name" model.name Name
    , viewInput "password" "Password" model.password Password
    , viewInput "password" "Re-enter Password" model.passwordAgain PasswordAgain
    , viewValidation model
    ]


viewInput : String -> String -> String -> (String -> msg) -> Html msg
viewInput t p v toMsg =
  input [ type_ t, placeholder p, value v, onInput toMsg ] []


viewValidation : Model -> Html msg
viewValidation model =
  if model.password == model.passwordAgain then
    div [ style "color" "green" ] [ text "OK" ]
  else
    div [ style "color" "red" ] [ text "Passwords do not match!" ]
```

## Types

* `type inference`编译器会快速分析源码以及数据的流动（运行前），非法会迅速报错

### Reading Types

#### Primitives and Lists

#### Functions

```
String.length
<function>:String -> Int
```

* ->是多个参数的分隔符

#### Type Annotations（类型注解）

```elm
hypotenuse : Float -> Float -> Float
hypotenuse a b =
  sqrt (a^2 + b^2)

-- hypotenuse 3 4  == 5
-- hypotenuse 5 12 == 13
```

* 不是必要的，但是写上去有很多好处
  * 编译器会检查结果类型是否匹配
  * 有助于别人理解函数的目标

#### Type Variables（可变类型）

```
> List.length
<f>: List a -> Int
```

* a：可以匹配任何类型，因为length并不关心List内Values的类型

![image-20200627201944018](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200627201944018.png)

* Type Variables必须以大写字母开头

#### Constrained Type Variables

The full list of constrained type variables is:

- `number` permits `Int` and `Float`
- `appendable` permits `String` and `List a`
- `comparable` permits `Int`, `Float`, `Char`, `String`, and lists/tuples of `comparable` values
- `compappend` permits `String` and `List comparable`

这些类型使得+和<更加灵活

### Type Aliases（类型别名）

```elm
type alias User =
  { name : String
  , age : Int
  }
```

* 类型别名是比类型更短的名字

#### Models

### Custom Types

对比

```elm
type UserStatus
  = Regular
  | Visitor

type alias User =
  { status : UserStatus
  , name : String
  }

thomas = { status = Regular, name = "Thomas" }
kate95 = { status = Visitor, name = "kate95" }
```

```elm
type User
  = Regular String
  | Visitor String

thomas = Regular "Thomas"
kate95 = Visitor "kate95"
```

第二种的好处

* 数据直接和变量相关联
* 每个变量可以和不同的data相关联

#### Messages

```elm
type Msg
  = PressedEnter
  | ChangedDraft String
  | ReceivedMessage { user : User, message : String }
  | ClickedExit
```

#### Modeling

```elm
type Profile
  = Failure
  | Loading
  | Success { name : String, description : String }
```

### Pattern

#### case

toname-function

```elm
toName : User -> String
toName user =
  case user of
    Regular name age ->
      name

    Visitor name ->
      name

-- toName (Regular "Thomas" 44) == "Thomas"
-- toName (Visitor "kate95")    == "kate95"
```



#### Wild Cards`_`

When some of the associated data is unused, it is common to use a “wild card” instead of giving it a name:

```elm
toName : User -> String
toName user =
  case user of
    Regular name _ ->
      name

    Visitor name ->
      name
```

## Error Handling

* Elm将错误看作一种数据

```elm
type MaybeAge
  = Age Int
  | InvalidInput

toAge : String -> MaybeAge
toAge userInput =
  ...

-- toAge "24" == Age 24
-- toAge "99" == Age 99
-- toAge "ZZ" == InvalidInput
```

```elm
type MaybePost
  = Post { title : String, content : String }
  | NoTitle
  | NoContent

toPost : String -> String -> MaybePost
toPost title content =
  ...

-- toPost "hi" "sup?" == Post { title = "hi", content = "sup?" }
-- toPost ""   ""     == NoTitle
-- toPost "hi" ""     == NoContent
```

* 当输入为空或者缺少输入的值时也不会报错

### Maybe

#### Partial Functions

```elm
type Maybe a
  = Just a
  | Nothing

-- Just 3.14 : Maybe Float
-- Just "hi" : Maybe String
-- Just True : Maybe Bool
-- Nothing   : Maybe a
```

![image-20200702104225742](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200702104225742.png)

#### Optional Fields

#### Avoiding Overuse

对比

```elm
type alias Friend =
  { name : String
  , age : Maybe Int
  , height : Maybe Float
  , weight : Maybe Float
  }
```

```elm
type Friend
  = Less String
  | More String Info

type alias Info =
  { age : Int
  , height : Float
  , weight : Float
  }
```

附：

Elm不具备null，而是使用Maybe使得错误更加明确（和Nothing有什么关系呢）

### Result

```elm
type Result error value
  = Ok value
  | Err error
```

用来存储错误信息

#### Error Reporting

```elm
isReasonableAge : String -> Result String Int
isReasonableAge input =
  case String.toInt input of
    Nothing ->
      Err "That is not a number!"

    Just age ->
      if age < 0 then
        Err "Please try again after you are born."

      else if age > 135 then
        Err "Are you some kind of turtle?"

      else
        Ok age

-- isReasonableAge "abc" == Err ...
-- isReasonableAge "-13" == Err ...
-- isReasonableAge "24"  == Ok 24
-- isReasonableAge "150" == Err ...
```



#### Error Recovery

```elm
type Error
  = BadUrl String
  | Timeout
  | NetworkError
  | BadStatus Int
  | BadBody String

-- Ok "All happy ..." : Result Error String
-- Err Timeout        : Result Error String
-- Err NetworkError   : Result Error String
```

根据错误的信息来进行相应的修正

## Commands and Subscription

#### Sandbox

* 目前为止项目都是用`Browser.sandbox`来创建的

![img](https://guide.elm-lang.org/effects/diagrams/sandbox.svg)

Runtime System：使得发送Html更加高效-Elm’s runtime system is a big part of why Elm is [one of the fastest options available](https://elm-lang.org/blog/blazing-fast-html-round-two)

DOM：当有点击或者文本框输入时发送Msg

#### element

* 在后面的例子中将会使用`Browser.element`来创建项目，包括commands和subscriptions

  ![img](https://guide.elm-lang.org/effects/diagrams/element.svg)

为了产生Html values，项目会发送Cmd和Sub Values给Runtime system，从而产生一个HTTP请求或者生成一个随机数、获得时间

#### dependence（packages）

* elm/core
* elm/html
* elm/json
* eml/http
* elm/random
* elm/time

对于本地项目，可能需要运行如下语句

```elm
elm init
elm install elm/xxx
```

### HTTP

加载一本书到网页上，用到elm/http

```elm
import Browser
import Html exposing (Html, text, pre)
import Http



-- MAIN


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }



-- MODEL


type Model
  = Failure
  | Loading
  | Success String


init : () -> (Model, Cmd Msg)
init _ =
  ( Loading
  , Http.get
      { url = "https://elm-lang.org/assets/public-opinion.txt"
      , expect = Http.expectString GotText
      }
  )



-- UPDATE


type Msg
  = GotText (Result Http.Error String)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    GotText result ->
      case result of
        Ok fullText ->
          (Success fullText, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  case model of
    Failure ->
      text "I was unable to load your book."

    Loading ->
      text "Loading..."

    Success fullText ->
      pre [] [ text fullText ]
```

#### init

```elm
init : () -> (Model, Cmd Msg)
init _ =
  ( Loading
  , Http.get
      { url = "https://elm-lang.org/assets/public-opinion.txt"
      , expect = Http.expectString GotText
      }
  )
```

Loading state,Get请求，确定url，显示在屏幕上的是一个大的String

Http.expectString GotText说明请求String的同时也会接受response并且转化为GotText Meg

```elm
type Msg
  = GotText (Result Http.Error String)

-- GotText (Ok "The Project Gutenberg EBook of ...")
-- GotText (Err Http.NetworkError)
-- GotText (Err (Http.BadStatus 404))
```

init实际上是一个函数

#### update

```elm
update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    GotText result ->
      case result of
        Ok fullText ->
          (Success fullText, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)
```

不仅返回了一个update model，而且生成了一个command

Cmd.none：用来指明没有更多的工作需要做，即本例中所有的文件已经被加载

#### subscription

此处暂时不需要

#### summary

![img](https://guide.elm-lang.org/effects/diagrams/element.svg)

通过`init`和`update`发布commands，由此可以发送HTTP请求，而且我们也可以获得更多有趣的信息

### JSON

* 在实际应用中，服务器返回的数据是特定格式（JSON）

```elm
import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode exposing (Decoder, field, string)



-- MAIN


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }



-- MODEL


type Model
  = Failure
  | Loading
  | Success String


init : () -> (Model, Cmd Msg)
init _ =
  (Loading, getRandomCatGif)



-- UPDATE


type Msg
  = MorePlease
  | GotGif (Result Http.Error String)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    MorePlease ->
      (Loading, getRandomCatGif)

    GotGif result ->
      case result of
        Ok url ->
          (Success url, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text "Random Cats" ]
    , viewGif model
    ]


viewGif : Model -> Html Msg
viewGif model =
  case model of
    Failure ->
      div []
        [ text "I could not load a random cat for some reason. "
        , button [ onClick MorePlease ] [ text "Try Again!" ]
        ]

    Loading ->
      text "Loading..."

    Success url ->
      div []
        [ button [ onClick MorePlease, style "display" "block" ] [ text "More Please!" ]
        , img [ src url ] []
        ]



-- HTTP


getRandomCatGif : Cmd Msg
getRandomCatGif =
  Http.get
    { url = "https://api.giphy.com/v1/gifs/random?api_key=dc6zaTOxFJmzC&tag=cat"
    , expect = Http.expectJson GotGif gifDecoder
    }


gifDecoder : Decoder String
gifDecoder =
  field "data" (field "image_url" string)
```

* init：进入Loading状态，获取随机的动态猫图
* update：当新的GIF可用时，处理GotGif的信息；处理MorePlease按钮的信息，更新图片
* view：显示GIF
* 区别：用到了`Http.expectJson`

#### JSON

当向服务器发送获取GIF的请求后，服务器会产生如下的JSON字符串

```json
{
  "data": {
    "type": "gif",
    "id": "l2JhxfHWMBWuDMIpi",
    "title": "cat love GIF by The Secret Life Of Pets",
    "image_url": "https://media1.giphy.com/media/l2JhxfHWMBWuDMIpi/giphy.gif",
    "caption": "",
    ...
  },
  "meta": {
    "status": 200,
    "msg": "OK",
    "response_id": "5b105e44316d3571456c18b3"
  }
}
```

* 在JS中，JSON会转化为JS obj,如果出现错误，会抛出runtime exception的异常
* 在Elm中，会在运行前验证JSON

#### JSON Decoders

```json
{
    "name": "Tom",
    "age": 42
}
```

![img](https://guide.elm-lang.org/effects/diagrams/int.svg)

![img](https://guide.elm-lang.org/effects/diagrams/string.svg)

#### Building Blocks

在elm/json包中使用`Json.Decode`module

```elm
import Json.Decode exposing (Decoder, field, int)

ageDecoder : Decoder Int
ageDecoder =
  field "age" int

 -- int : Decoder Int
 -- field : String -> Decoder a -> Decoder a
```

```elm
import Json.Decode exposing (Decoder, field, string)

nameDecoder : Decoder String
nameDecoder =
  field "name" string

-- string : Decoder String
```

可以看到，`field`有两个参数：

1.String-a field name

2.Decoder a-a decoder to try next

功能：`field “age” int`--访问“age”field，如果存在，则运行`Decoder Int`从而提取出一个整数

#### Nesting Decoders

以random cat GIF为例

### Random

### Time

