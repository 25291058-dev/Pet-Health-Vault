# Reporte Tecnico - Pet Health Vault

Lógica del Programa:
El contrato permite registrar mascotas y guardar su historial clinico en la blockchain. Se uso el framework Anchor para la seguridad.

Puntos clave:
* PDAs: Se usaron Program Derived Addresses para que cada dueño tenga una cuenta unica basada en su llave publica.
* Espacio: Se limito el historial a 10 consultas para controlar el costo de renta en Solana.
* Seguridad: Solo el dueño que registro a la mascota puede agregar nuevas consultas medicas.

Nota sobre las pruebas:
Para que el programa funcione, primero se debe inicializar la cuenta con registrarMascota. Si se intenta usar otra funcion antes, el programa dara un error de cuenta no inicializada.
