import numpy as np
from PIL import Image
import struct

if __name__ == "__main__":
    path = "/mnt/c/screenshots_and_videos/wanderer_hat.PNG"
    image = Image.open(path)
    a = np.array(image)

    vd = np.ones((1, a.shape[0], a.shape[1], 4), dtype=np.uint8)
    vd[0, :, :, 1:] = a[:, :, :3]

    # np.shape is backwards from our coordinate convention, = (z, y, x, voxel)
    # np.array[z][y][x][voxel]

    with open("/mnt/c/screenshots_and_videos/wanderer_hat.vd", "wb") as f:
        # > is big-endian
        b = struct.pack(">iii", vd.shape[2], vd.shape[1], vd.shape[0])
        f.write(b)
        for z in range(vd.shape[0]):
            for y in range(vd.shape[1]):
                for x in range(vd.shape[2]):
                    b = struct.pack(
                        ">ifff",
                        vd[z, y, x, 0],
                        vd[z, y, x, 1] / 255.0,
                        vd[z, y, x, 2] / 255.0,
                        vd[z, y, x, 3] / 255.0,
                    )
                    print(
                        vd[z, y, x, 0],
                        vd[z, y, x, 1] / 255.0,
                        vd[z, y, x, 2] / 255.0,
                        vd[z, y, x, 3] / 255.0,
                    )

                    f.write(b)
