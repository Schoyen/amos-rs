import numpy as np
import scipy.special

np.random.seed(2022)

n = 3

nu_rp = np.random.random() + 0.1
nu_rm = np.random.random() + 0.1

nu_start = [
    0,
    -1,
    nu_rp,
    -nu_rm,
]

m = len(nu_start) * n


out1 = np.zeros((m, 6))
out1_e = np.zeros_like(out1)
out2 = np.zeros((m, 6))
out2_e = np.zeros_like(out1)


for i, nu_0 in enumerate(nu_start):
    zr = (-1) ** (np.random.randint(2)) * np.random.random()
    zi = (-1) ** (np.random.randint(2)) * np.random.random()

    for j in range(n):
        cy1 = scipy.special.hankel1(
            (nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi
        )
        cy1_e = scipy.special.hankel1e(
            (nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi
        )
        cy2 = scipy.special.hankel2(
            (nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi
        )
        cy2_e = scipy.special.hankel2e(
            (nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi
        )

        out1[i * n + j, 0] = nu_0
        out1[i * n + j, 1] = j if nu_0 >= 0 else -j
        out1[i * n + j, 2] = zr
        out1[i * n + j, 3] = zi
        out1[i * n + j, 4] = cy1.real
        out1[i * n + j, 5] = cy1.imag

        out1_e[i * n + j, 0] = nu_0
        out1_e[i * n + j, 1] = j if nu_0 >= 0 else -j
        out1_e[i * n + j, 2] = zr
        out1_e[i * n + j, 3] = zi
        out1_e[i * n + j, 4] = cy1_e.real
        out1_e[i * n + j, 5] = cy1_e.imag

        out2[i * n + j, 0] = nu_0
        out2[i * n + j, 1] = j if nu_0 >= 0 else -j
        out2[i * n + j, 2] = zr
        out2[i * n + j, 3] = zi
        out2[i * n + j, 4] = cy2.real
        out2[i * n + j, 5] = cy2.imag

        out2_e[i * n + j, 0] = nu_0
        out2_e[i * n + j, 1] = j if nu_0 >= 0 else -j
        out2_e[i * n + j, 2] = zr
        out2_e[i * n + j, 3] = zi
        out2_e[i * n + j, 4] = cy2_e.real
        out2_e[i * n + j, 5] = cy2_e.imag


np.savetxt("zbesh1_test.txt", out1, delimiter=" ", header="nu j zr zi cyr cyi")
np.savetxt("zbesh1_e_test.txt", out1_e, delimiter=" ", header="nu j zr zi cyr cyi")
np.savetxt("zbesh2_test.txt", out2, delimiter=" ", header="nu j zr zi cyr cyi")
np.savetxt("zbesh2_e_test.txt", out2_e, delimiter=" ", header="nu j zr zi cyr cyi")
