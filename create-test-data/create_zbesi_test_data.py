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


out = np.zeros((m, 6))
out_e = np.zeros_like(out)


for i, nu_0 in enumerate(nu_start):
    for j in range(n):
        zr = (-1) ** (np.random.randint(2)) * np.random.random()
        zi = (-1) ** (np.random.randint(2)) * np.random.random()
        cy = scipy.special.iv((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi)
        cy_e = scipy.special.ive((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi)

        out[i * n + j, 0] = nu_0
        out[i * n + j, 1] = j if nu_0 >= 0 else -j
        out[i * n + j, 2] = zr
        out[i * n + j, 3] = zi
        out[i * n + j, 4] = cy.real
        out[i * n + j, 5] = cy.imag

        out_e[i * n + j, 0] = nu_0
        out_e[i * n + j, 1] = j if nu_0 >= 0 else -j
        out_e[i * n + j, 2] = zr
        out_e[i * n + j, 3] = zi
        out_e[i * n + j, 4] = cy_e.real
        out_e[i * n + j, 5] = cy_e.imag


np.savetxt("zbesi_test.txt", out, delimiter=", ", header="nu, j, zr, zi, cyr, cyi")
np.savetxt("zbesi_e_test.txt", out_e, delimiter=", ", header="nu, j, zr, zi, cyr, cyi")
