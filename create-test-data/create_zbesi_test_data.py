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

out = []
out_e = []
out_real = []
out_e_real = []


for i, nu_0 in enumerate(nu_start):
    zr = (-1) ** (np.random.randint(2)) * np.random.random()
    zi = (-1) ** (np.random.randint(2)) * np.random.random()

    for j in range(n):
        cy = scipy.special.iv((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi)
        cy_e = scipy.special.ive((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr + 1j * zi)
        cy_real = scipy.special.iv((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr)
        cy_e_real = scipy.special.ive((nu_0 + j) if nu_0 >= 0 else (nu_0 - j), zr)

        assert cy_real.dtype == float
        assert cy_e_real.dtype == float

        out.append([nu_0, j if nu_0 >= 0 else -j, zr, zi, cy.real, cy.imag])
        out_e.append([nu_0, j if nu_0 >= 0 else -j, zr, zi, cy_e.real, cy_e.imag])

        if np.floor(nu_0) != nu_0 and zr < 0:
            assert np.isnan(cy_real)
            assert np.isnan(cy_e_real)

            continue

        out_real.append([nu_0, j if nu_0 >= 0 else -j, zr, cy_real])
        out_e_real.append([nu_0, j if nu_0 >= 0 else -j, zr, cy_e_real])


np.savetxt(
    "zbesi_test.txt", np.asarray(out), delimiter=" ", header="nu j zr zi cyr cyi"
)
np.savetxt(
    "zbesi_e_test.txt", np.asarray(out_e), delimiter=" ", header="nu j zr zi cyr cyi"
)

np.savetxt(
    "zbesi_real_test.txt", np.asarray(out_real), delimiter=" ", header="nu j z cy"
)
np.savetxt(
    "zbesi_e_real_test.txt", np.asarray(out_e_real), delimiter=" ", header="nu j z cy"
)
