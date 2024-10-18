import matplotlib.pyplot as plt
import numpy as np
from scipy.spatial import Delaunay
from matplotlib.path import Path

# 定义点集合
new_points = np.array([
    [74, 21], [71, 24], [70, 24], [64, 30], [64, 31], [62, 33], [58, 41], [58, 43], [56, 47], [56, 51],
    [55, 52], [55, 60], [54, 61], [54, 160], [151, 160], [151, 127], [150, 126], [150, 24], [148, 24],
    [147, 23], [145, 23], [144, 22], [142, 22], [138, 20], [135, 20], [131, 18], [128, 18], [127, 17],
    [123, 17], [122, 16], [117, 16], [116, 15], [105, 15], [104, 14], [86, 16], [85, 17], [80, 18], [74, 21]
])

# 创建一个路径对象，包含轮廓的全部点
contour_path = Path(new_points)

# 创建一个绘制图形的函数
def plot_triangulation(tri, new_points, contour_path, step_delay=1):
    # 创建一个空图形
    fig, ax = plt.subplots(figsize=(10, 10), facecolor='lightgrey')

    # 设置坐标轴的颜色和标签
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_title('Constrained Delaunay Triangulation within Contour')

    # 反转Y轴
    ax.invert_yaxis()

    # 绘制轮廓的点
    ax.plot(new_points[:, 0], new_points[:, 1], 'o', color='red', markersize=5)

    # 关闭轮廓线，使最后一个点和第一个点相连
    ax.plot([new_points[-1, 0], new_points[0, 0]], [new_points[-1, 1], new_points[0, 1]], 'darkblue', marker='o',
            markersize=5, markerfacecolor='red')

    for i in range(1, len(new_points)):
        ax.plot([new_points[i - 1, 0], new_points[i, 0]], [new_points[i - 1, 1], new_points[i, 1]], 'darkblue')

    # 显示轮廓的线
    ax.plot(new_points[:, 0], new_points[:, 1], 'darkblue')

    # 刷新图形
    plt.draw()
    plt.pause(step_delay)

    # 计算Delaunay三角剖分
    tri = Delaunay(new_points)

    # 找到所有三角形的中点
    tri_centers = np.mean(new_points[tri.simplices], axis=1)

    # 确定中点是否在轮廓路径内部
    is_inside = contour_path.contains_points(tri_centers)

    # 过滤掉不在轮廓内部的三角形
    tri_simplices_inside = tri.simplices[is_inside]

    # 绘制轮廓内部的三角剖分
    ax.triplot(new_points[:, 0], new_points[:, 1], tri_simplices_inside, color='darkblue')

    # 刷新图形
    plt.draw()
    plt.pause(step_delay)

    # 显示图形
    plt.show()

# 调用绘制函数
plot_triangulation(Delaunay(new_points), new_points, contour_path)
