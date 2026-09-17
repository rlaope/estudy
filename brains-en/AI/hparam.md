# Hyperparameters

Before diving into hyperparameters, let's define model parameters.

These are variables whose values are determined by the model itself during the training process. Generally, they are the values fed into the model.

Examples include weights `w` and bias `b` in linear regression. They are updated through optimization processes like gradient descent, based on the data and the loss function.

For example, variables that define relationships such as '1 hour of study yields 20 points' or '2 hours of study yields 30 points' are model parameters.

### Hyperparameters

These are values specified directly by humans outside the model, meaning the model cannot learn them on its own.

Examples include the learning rate (weight update size), batch size (number of samples processed at once), and epochs (how many times the entire dataset is iterated over).

Other hyperparameters include the number of hidden layers and neurons (in neural networks), regularization coefficients to prevent overfitting, and optimizers.

The difference from model parameters is that the model cannot learn them itself; they are determined externally.

Optimal values are found using techniques like greedy search, random search, or Bayesian optimization.

For hyperparameter tuning strategies, it's good to check performance and adjust using small values, log-scale search, and cross-validation, among other methods.
