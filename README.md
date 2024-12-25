# code-annotations
Annotate code with notes

## File format
Notes are stored in a format based on patch/diff with context to locate lines
after modifications

```
<128-bit UUID>
<relative-filepath>
<line number> <m: number of context lines before> <n: number of context lines after>
<note-text>
<m context lines>
<line with note>
<m context lines>
```

The file path is relative to the project root. This is usually the git root, but can be user specified.

The note text cannot contain newlines.

### Example
```
214788ce-f405-4fd3-bd43-ad1d4a0c006f
mathlib.c
55 3 3
https://github.com/francisrstokes/githublog/blob/main/2024/5/29/fast-inverse-sqrt.md
x2 = number * 0.5F;
y  = number;
i  = *(long*)&y;
i  = 0x5f3759df - ( i >> 1 );
y  = *(float*)&i;
y  = y * ( threehalfs - ( x2 * y * y ) );
// y  = y * ( threehalfs - ( x2 * y * y ) );
```

## Non-features

- Linking between notes
- Extended character support and newlines in the note text
