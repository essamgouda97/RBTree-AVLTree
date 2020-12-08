
"""
========
Barchart
========

A bar plot with height labels on individual bars
"""
import numpy as np
import matplotlib.pyplot as plt

N = 5
rb_res = (1.0642, 5.9522, 11.708, 18.242, 25.206)


ind = np.arange(N)  # the x locations for the groups
width = 0.35       # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind, rb_res, width, color='r')

avl_res = ()

rects2 = ax.bar(ind + width, avl_res, width, color='y')

# add some text for labels, title and axes ticks
ax.set_ylabel('Time (ms)')
ax.set_title('Benchmark results Insertion')
ax.set_xticks(ind + width / 2)
ax.set_xticklabels(('10000', '40000', '70000', '100000', '130000'))

ax.legend((rects1[0], rects2[0]), ('RBTree', 'AVL'))


def autolabel(rects):
    """
    Attach a text label above each bar displaying its height
    """
    for rect in rects:
        height = rect.get_height()
        ax.text(rect.get_x() + rect.get_width()/2., height,
                '%d' % int(height),
                ha='center', va='bottom')

autolabel(rects1)
autolabel(rects2)

plt.show()
