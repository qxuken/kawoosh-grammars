(ns sample.core
  (:require [clojure.string :as str]))

;; A sample.
(def limit 10)

(defrecord Point [x y])

(defprotocol Shape
  (area [this]))

(defrecord Circle [radius]
  Shape
  (area [_] (* Math/PI radius radius)))

(defn length
  "The point's distance from the origin."
  [{:keys [x y]}]
  (Math/sqrt (+ (* x x) (* y y))))

(defn largest [items]
  (when (seq items)
    (reduce max items)))

(defmulti describe :kind)
(defmethod describe :circle [shape] (str "a circle of " (:radius shape)))
(defmethod describe :default [_] "nothing")

(defn -main [& args]
  (let [points [(->Point 3 4) (->Point 1 0)]
        lengths (->> points (map length) (filter #(> % 1)))]
    (doseq [[i l] (map-indexed vector lengths)]
      (println i (format "%.2f" l)))
    (try
      (println (largest [1 2 3]) (str/join ", " args) #{:a :b} {:k "v"})
      (catch Exception e
        (throw (ex-info "failed" {:cause e}))))
    (when-let [n (some-> (first args) Integer/parseInt)]
      (cond
        (< n limit) :small
        :else :large))))
