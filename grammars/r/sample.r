# A sample.
library(stats)

limit <- 10

area <- function(shape) {
  if (shape$kind == "circle") {
    pi * shape$radius^2
  } else if (shape$kind == "rect" && shape$width > 0) {
    shape$width * shape$height
  } else {
    0
  }
}

largest <- function(items) {
  if (length(items) == 0) {
    return(NULL)
  }
  max(items)
}

shapes <- list(
  list(kind = "circle", radius = 1),
  list(kind = "rect", width = 2, height = 3)
)
areas <- sapply(shapes, area)
small <- areas[areas < limit]

for (i in seq_along(small)) {
  cat(sprintf("%d: %.2f\n", i, small[[i]]))
}

df <- data.frame(x = c(3, 1), y = c(4, 0))
df$length <- sqrt(df$x^2 + df$y^2)
result <- tryCatch(
  as.integer("42") + nrow(df),
  warning = function(w) NA_integer_
)
print(largest(c(result, TRUE, 3L)))
