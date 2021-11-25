## Cut then inc

```
((((ptr + n)) * pow(inc, -1, mod=decksize)))
```

## Inc then cut

```
(((ptr * pow(inc, -1, mod=decksize)) + c))
```

## Cut then deal

```
(decksize - ((ptr + n)) - 1)
```

## Deal then cut

```
((decksize - ptr - 1 + n))
```

## Deal then Inc

```
(((decksize - ptr - 1) * pow(inc, -1, mod=decksize)))
```

## Inc then deal

```
(decksize - ((ptr * pow(inc, -1, mod=decksize))) - 1)
```