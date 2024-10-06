# JSON data
import json

with open('newoutput2.json', 'r') as f:
    data = json.load(f)

with open('input2.json', 'r') as f:
    click_data = json.load(f)
def points_to_svg(points, color, width=1):
    svg_points = " ".join([f"{p['lon']},{p['lat']}" for p in points])
    return f'<polygon points="{svg_points}" style="fill:none;stroke:{color};stroke-width:{width}" />'

def calculate_viewbox(data):
    all_points = data["parent"]["contour_points"] + [pt for child in data["children"] for pt in child["contour_points"]]
    min_x = min(p["lon"] for p in all_points)
    min_y = min(p["lat"] for p in all_points)
    max_x = max(p["lon"] for p in all_points)
    max_y = max(p["lat"] for p in all_points)
    return min_x, min_y, max_x - min_x, max_y - min_y

if __name__ == "__main__":
    min_x, min_y, width, height = calculate_viewbox(data)
    # Generate SVG content
    svg_content = f'<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {912} {1230}">\n'
    svg_content += points_to_svg(data["parent"]["contour_points"], "red", 1) + "\n"
    for child in data["children"]:
        svg_content += points_to_svg(child["contour_points"], "blue", 1) + "\n"
    for click in click_data["right_clicks"]:
        svg_content += f'<circle cx="{click["x"]}" cy="{click["y"]}" r="5" fill="green" />\n'
    svg_content += '</svg>'

    # Write to SVG file
    with open("contours.svg", "w") as f:
        f.write(svg_content)

    print("SVG file generated: contours.svg")