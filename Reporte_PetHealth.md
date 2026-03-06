# Reporte Técnico: Pet Health Vault

## Almacenamiento y PDAs
Este contrato utiliza **Program Derived Addresses (PDAs)** para asegurar que cada mascota esté vinculada de forma única a su dueño.

### Estructura de la PDA
La dirección de la cuenta de la mascota se deriva utilizando:
- La semilla (seed) estática: `"expediente"`
- La llave pública del dueño (`owner`)

### Gestión de Espacio
Se ha calculado el espacio necesario para soportar:
- **Nombre y Raza**: Cadenas de texto con límites definidos.
- **Historial**: Un vector que permite almacenar múltiples consultas médicas, optimizando el uso de memoria en la red de Solana.

