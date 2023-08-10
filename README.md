# Presupuestos
Este es un proyecto personal para practicar tanto Rust como ejercicios de contabilidad.
El programa pretende servir para crear, interpretar y corregir asientos contables, ateniéndose al [Plan General de Contabilidad](https://www.boe.es/buscar/act.php?id=BOE-A-2007-19884) actualmente en vigor en España.

## Características principales
El programa se sirve fundamentalmente de la línea de comandos.
Necesita dos documentos para funcionar correctamente:
1. ***cuadro.txt**: De aquí recoge los códigos y cuentas. Será básicamente una copia del cuadro del PGC, con la flexibilidad añadida de que se podrán añadir o quitar cuentas a medida que sea necesario.
2. **diario/**: En esta carpeta se encuentran los asientos, que se nombran mediante la convención FECHANÚMERO.data. Dentro de cada uno de estos documentos, sigo la siguiente estructura:

```
<Descripción del asiento>

DEBE
<Código de cuenta> <Importe>
<Código de cuenta> <Importe>

HABER
<Código de cuenta> <Importe>

///
```
### Balance de situación
+ Puede crear un balance de situación inicial si existe un documento llamado **balance_inicial.txt** en el mismo directorio La estructura es la siguiente:
```
<Masa patrimonial>
<Código de cuenta> <Importe>
```
En este caso