import matplotlib.pyplot as plt
import numpy as np
from matplotlib.path import Path
from scipy.spatial import Delaunay

contour_b = np.array([
    [150, 120],
    [105, 184],
    [115, 188],
    [118, 188],
    [125, 191],
    [136, 193],
    [151, 198],
    [165, 205],
    [186, 221],
    [190, 226],
    [192, 226],
    [234, 187],
    [238, 182],
    [205, 140],
    [200, 137],
    [186, 132],
    [171, 129],
    [159, 125],
    [150, 120]
])
contour_a = np.array([
    [269, 56],
    [239, 64],
    [191, 85],
    [168, 91],
    [126, 94],
    [84, 89],
    [59, 231],
    [158, 265],
    [187, 259],
    [249, 233],
    [268, 231],
    [269, 56],
])
new_points = np.concatenate([contour_a, contour_b])

# 计算Delaunay三角剖分
tri = Delaunay(new_points)
# 创建一个路径对象，包含轮廓的全部点
contour_path = Path(contour_b)
contour_path2 = Path(new_points)
# 找到所有三角形的中点
tri_centers = np.mean(new_points[tri.simplices], axis=1)

# 确定中点是否在小轮廓路径内部
is_inside = contour_path.contains_points(tri_centers)
# 确定中点是否在大轮廓路径内部
is_inside2 = contour_path2.contains_points(tri_centers)

# 过滤掉在小轮廓内部的三角形与大轮廓外部的三角形
tri_simplices = tri.simplices[~is_inside*is_inside2]
print(len(tri_simplices))
# 绘制轮廓内部的三角剖分
plt.figure(figsize=(10, 10), facecolor='lightgrey')

# 绘制轮廓的三角剖分
plt.triplot(new_points[:, 0], new_points[:, 1], tri_simplices, color='darkblue')
# plt.triplot(new_points[:, 0], new_points[:, 1], tri.simplices.copy(), color='darkblue')

# 绘制轮廓的点
plt.plot(new_points[:, 0], new_points[:, 1], 'o', color='red', markersize=5)

# 设置坐标轴的颜色和标签
plt.xlabel('X')
plt.ylabel('Y')
plt.title('Constrained Delaunay Triangulation within Contour')

# 反转Y轴
plt.gca().invert_yaxis()

# 显示图形
plt.show()
