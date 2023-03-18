# rusty-compress

### A small decompresser built using zip crate

**OUTPUT**

```rust
$ cargo run org.zip
    Compiling rusty-decompress v0.1.0 (D:\vscode-library\rust-gallery\rusty-decompress)
      Finished dev [unoptimized + debuginfo] target(s) in 7.21s
      Running `target\debug\rusty-decompress.exe org.zip`
  File 0 extracted to "org/New Bitmap Image.bmp" ( 0 bytes)
  File 1 extracted to "org/New folder/"
  File 2 extracted to "org/New folder/New folder/"
  File 3 extracted to "org/New folder/New folder/New Text Document.txt" ( 0 bytes)
  File 4 extracted to "org/New folder/New Microsoft Word Document.docx" ( 0 bytes)
  File 5 extracted to "org/New folder/New Text Document.txt" ( 0 bytes)
  File 6 extracted to "org/New Microsoft Excel Worksheet.xlsx" ( 8746 bytes)cargo run original-file.pdf compressed-file
```
