(ns bowling.frame-test
  (:require [clojure.test :refer :all]
            [bowling.frame :refer :all]))

(deftest frame-test
  (testing "frame is generated as empty vector"
    (is (vector? (start-game)))
    (is (= 0 (count (start-game))))))
