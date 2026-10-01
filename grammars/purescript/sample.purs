module Sample where

import Prelude

import Data.Array (filter, length)
import Data.Maybe (Maybe(..))
import Effect (Effect)
import Effect.Console (log)

-- | A sample.
data Shape
  = Circle Number
  | Rect Number Number
  | Empty

type Point = { x :: Number, y :: Number }

newtype Name = Name String

class Named a where
  name :: a -> String

instance namedShape :: Named Shape where
  name (Circle _) = "circle"
  name (Rect _ _) = "rect"
  name Empty = "empty"

limit :: Int
limit = 10

area :: Shape -> Number
area (Circle r) = 3.14159 * r * r
area (Rect w h)
  | w > 0.0 = w * h
  | otherwise = 0.0
area Empty = 0.0

largest :: Array Int -> Maybe Int
largest items = case items of
  [] -> Nothing
  _ -> Just (length items)

main :: Effect Unit
main = do
  let
    shapes = [ Circle 1.0, Rect 2.0 3.0, Empty ]
    small = filter (\s -> area s < 10.0) shapes
  -- A comment.
  log (show (length small))
  case largest [ 1, 2, 3 ] of
    Just n -> log ("largest: " <> show (n + limit))
    Nothing -> pure unit
