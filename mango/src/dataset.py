import numpy as np


def create_spiral_dataset(samples, classes, fp):
    f = open(fp, "w+")
    for class_number in range(classes):
        r = np.linspace(0.0, 1, samples)
        t = np.linspace(class_number*4, (class_number+1)*4, samples) + np.random.randn(samples)*0.2

        i = r*np.sin(t * 2.5)
        j = r*np.cos(t * 2.5)

        for k in range(samples):
            f.write(str(class_number) + ":" + str(i[k]) + ";" + str(j[k]) + "\n")

    f.close()

create_spiral_dataset(100, 3, "spiral_data.mdsf")
