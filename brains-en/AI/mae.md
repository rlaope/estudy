# Regression Model Performance Metrics

When evaluating regression models, MAE, MSE, RMSE, and MAPE are commonly used, and today we'll explore them.

### MAE (Mean Absolute Error)

It calculates the average of the absolute differences between the actual values and the predicted values.

It is mainly used when there are many outliers (as the difference between each value becomes smaller).

Lower values are better (meaning a smaller error range).

$$
MAE = \frac{\sum |y - \hat{y}|}{n}
$$

**Advantages**
- Intuitive and has the same units as the actual and predicted values.

**Disadvantages**
- It's difficult to determine whether the prediction was lower or higher than the actual value.
- It is scale-dependent. Even if the error magnitude is the same across different models, the error rate may not be.

### MSE (Mean Squared Error)

It calculates the average of the squared differences between the actual values and the predicted values.

Lower values are better. The only advantage seems to be that it's intuitive.

$$
MSE = \frac{1}{n} \sum_{i=1}^{n} (y_i - \hat{y}_i)^2
$$

**Advantages**
- Intuitive.

**Disadvantages**
- Because it squares the errors, errors less than 1 become smaller, and errors greater than 1 become larger.
- It's difficult to determine whether the prediction was lower or higher than the actual value.
- It is scale-dependent. Even if the error magnitude is the same across different models, the error rate may not be.

### RMSE (Root Mean Squared Error)

By taking the square root of MSE, the distortion caused by squaring errors is reduced. Lower values are better.

$$
RMSE = \sqrt{\frac{1}{n} \sum_{i=1}^{n} (y_i - \hat{y}_i)^2}
$$

**Advantages**
- Intuitive.

**Disadvantages**
- Because it squares the errors, errors less than 1 become smaller, and errors greater than 1 become larger, just like MSE.
- It's difficult to determine whether the prediction was lower or higher than the actual value.
- Scale-dependent. (The rest of the content is the same)

### MAPE (Mean Absolute Percentage Error)

It expresses MAE as a ratio or percentage, improving upon the scale-dependent issue.

Lower values are better.

$$
MAPE = \frac{100}{n} \sum_{i=1}^{n} \left| \frac{y_i - \hat{y}_i}{y_i} \right|
$$

**Advantages**
- Intuitive and easy to compare error rates across different models.

**Disadvantages**
- It's difficult to determine whether the prediction was lower or higher than the actual value.
- If the actual value is less than 1, it can converge to an infinite value.

> The statement that metrics like MAE, MSE, and RMSE are scale-dependent means that the metric value itself increases or decreases with the unit (scale) of the data. For example, if an error of 500 is reported, but the unit (scale) is ten thousand won, it's difficult to tell from this value alone whether the error is 5 million won or 500 won.

In addition, MPE and R2 Score also exist, so let's explore them.

### MPE (Mean Percentage Error)

Excludes the absolute value from MAPE. It can determine whether the model is underperforming or overperforming; negative indicates overperformance, positive indicates underperformance.

$$
MPE = \frac{100}{n} \sum_{i=1}^{n} \frac{y_i - \hat{y}_i}{y_i}
$$

### R2 Score = R squared

MAE, MSE, and RMSE values vary across models, making it difficult to judge performance based solely on their absolute values.

R2 Score indicates relative performance, making comparisons easier. It represents the proportion of the variance in the predicted values that is explained by the actual values, and values closer to 1 are better.

$$
R^2 = 1 - \frac{SSE}{SST}
$$

$$
SSE = \frac{1}{n} \sum_{i=1}^{n} (y_i - \hat{y}_i)^2 = MSE
$$

$$
SST = \frac{1}{n} \sum_{i=1}^{n} (y_i - \bar{y})^2
$$
