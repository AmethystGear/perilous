import sys
from PIL import Image

def set_color(input_path, output_path, r, g, b):
    img = Image.open(input_path)
    img = img.convert("RGBA")
    pixel_data = img.getdata()

    new_img_data = [(r, g, b, pixel[3]) for pixel in pixel_data]
    new_img = Image.new("RGBA", img.size)
    new_img.putdata(new_img_data)

    new_img.save(output_path, "PNG")
    print(f"Image processed and saved to {output_path}")



input_image_path = sys.argv[1]
r = int(sys.argv[2])
g = int(sys.argv[3])
b = int(sys.argv[4])
output_image_path = "output_image.png"

set_color(input_image_path, output_image_path, r, g, b)
