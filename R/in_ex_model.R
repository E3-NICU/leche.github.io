library(ggplot2)

data360 <- read.csv("project_data/model360.csv")
data90 <- read.csv("project_data/model90.csv")

internal <- c(data360$internal_temp, data90$internal_temp)
external <- c(data360$external_temp, data90$external_temp)
volume <- c(data360$volume, data90$volume)
seconds <- c(data360$seconds, data90$seconds)

data <- data.frame(internal, external, volume, seconds)
data <- data[data$volume >= 20,]

model <- lm(internal ~ external + seconds, data = data)
summary(model)
AIC(model)
# plot(model)

predicts <- predict(model, interval = "prediction")
# predicts <- predicts + data360$external_temp
results <- cbind(data, predicts)

ggplot(data = data, aes(x = internal, y = external - internal, color = as.factor(volume))) +
  scale_colour_discrete("Volume") +
  theme_bw() +
  geom_point() +
  geom_smooth(method = "lm", se=FALSE)

ggplot(results, aes(x = internal, fit)) +
  geom_point(aes(x = internal, fit, color = as.factor(volume))) +
  scale_colour_discrete("Volume") +
  # geom_abline(intercept = 0, slope = 1) +
  geom_abline(intercept = 1, slope = 1) +
  geom_abline(intercept = -1, slope = 1) +
  theme_bw()
