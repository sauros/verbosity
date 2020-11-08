# verbosity
A simple configurable text output manager

## Macros to create verbosity objects with a particular level set
```
    verbosity_off!()
    verbosity_low!()
    verbosity_medium!()
    verbosity_high!()
```

## Macro to show verbosity iff verbosity > off
```
    verbosity_show!(&v);
```

## Macros to get current function signature
```
    vfunc!();
```

## Output Macros

```
    vlow!(v,  vfunc!(), "This will show iff level is at least low");
    vmed!(v,  vfunc!(), "This will show iff level is at least med");
    vhigh!(v, vfunc!(), "This will show only if level is high");
```


