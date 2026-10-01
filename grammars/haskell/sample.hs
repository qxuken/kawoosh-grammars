{-# LANGUAGE LambdaCase #-}
module Sample (Shape (..), area, main) where

import Data.List (sortBy)
import qualified Data.Map as Map

-- | A sample.
data Shape
  = Circle Double
  | Rect Double Double
  | Empty
  deriving (Show, Eq)

newtype Name = Name String

class Named a where
  name :: a -> String

instance Named Shape where
  name = \case
    Circle _ -> "circle"
    Rect _ _ -> "rect"
    Empty -> "empty"

area :: Shape -> Double
area (Circle r) = pi * r * r
area (Rect w h)
  | w > 0 = w * h
  | otherwise = 0
area Empty = 0

largest :: Ord a => [a] -> Maybe a
largest [] = Nothing
largest xs = Just (maximum xs)

main :: IO ()
main = do
  let shapes = [Circle 1, Rect 2 3, Empty]
      counts = Map.fromListWith (+) [(name s, 1 :: Int) | s <- shapes]
  -- A comment.
  mapM_ (print . area) (sortBy (\a b -> compare (area a) (area b)) shapes)
  case largest (map area shapes) of
    Just x -> putStrLn ("largest: " ++ show x)
    Nothing -> pure ()
  print (Map.toList counts)
