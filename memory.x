/* XMC4700-F144x2048: 2 MB Flash, Zugriff über den uncached Alias 0x0800_0000.
   RAM hier konservativ: DSRAM1 (erste 64 KB). */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 352K
}
