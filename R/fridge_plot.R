data <- read.csv("project_data/hospital_fridge_temp.csv")

hist(data$temp, breaks = 5, col = "lightblue", border = "white", xlab = "Temperature", main="Fridge temperature")

