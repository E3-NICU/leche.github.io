data <- read.csv("project_data/staff_warmup_temp.csv")

good_data <- data[data$microwave == "good",]
hist(good_data$temp, breaks = 20, col = "lightblue", border = "white", xlab = "Temperature", main="Good microwave")

bad_data <- data[data$microwave == "bad",]
hist(bad_data$temp, breaks = 10, col = "lightblue", border = "white", xlab = "Temperature", main="Bad microwave")