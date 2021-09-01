library(ggplot2)

data <- read.csv("project_data/model90.csv")
# data <- data[data$volume != 20, ]
data$diff <- (data$internal_temp - data$start_temp)

model <- lm(diff ~ seconds + volume, data = data)
summary(model)
AIC(model)
# plot(model)

predicts <- predict(model, interval = "prediction")
predicts <- predicts + data$start_temp
results <- cbind(data, predicts)

ggplot(data = data, aes(x = diff, y = seconds, color = as.factor(volume))) +
  scale_colour_discrete("Volume") +
  theme_bw() +
  geom_point() +
  geom_smooth(method = "lm")

ggplot(results, aes(x = internal_temp, fit)) +
  geom_point(aes(x = internal_temp, fit, color = as.factor(volume))) +
  scale_colour_discrete("Volume") +
  geom_abline(intercept = 0, slope = 1) +
  theme_bw()


