import pyrust_robot as rb
import numpy as np

# x = rb.array_add(np.array([2.3,1.2,5.2, 2.1]))
# x = rb.sum_as_string(2,3)

x = rb.LQR_control(np.array([0.3, 2.4, 1.2]))

print(x)
