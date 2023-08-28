use ansi_term::Style;
use phf::{phf_map, Map};

use crate::fs::File;

/// Mapping from full filenames to file type. This mapping should also contain all the "dot"
/// files/directories that have a custom icon.
const FILENAME_ICONS: Map<&'static str, char> = phf_map! {
    ".atom"               => '\u{e764}', // 
    ".bashprofile"        => '\u{e615}', // 
    ".bashrc"             => '\u{f489}', // 
    ".emacs"              => '\u{e632}', // 
    ".git"                => '\u{f1d3}', // 
    ".gitattributes"      => '\u{f1d3}', // 
    ".gitconfig"          => '\u{f1d3}', // 
    ".github"             => '\u{f408}', // 
    ".gitignore"          => '\u{f1d3}', // 
    ".gitignore_global"   => '\u{f1d3}', // 
    ".gitmodules"         => '\u{f1d3}', // 
    ".idea"               => '\u{e7b5}', // 
    ".rvm"                => '\u{e21e}', // 
    ".Trash"              => '\u{f1f8}', // 
    ".vimrc"              => '\u{e7c5}', // 
    ".vscode"             => '\u{e70c}', // 
    ".zshrc"              => '\u{f489}', // 
    "bin"                 => '\u{e5fc}', // 
    "Cargo.lock"          => '\u{e7a8}', // 
    "config"              => '\u{e5fc}', // 
    "docker-compose.yml"  => '\u{f308}', // 
    "Dockerfile"          => '\u{f308}', // 
    "ds_store"            => '\u{f179}', // 
    "Earthfile"           => '\u{f0ac}', // 
    "gitignore_global"    => '\u{f1d3}', // 
    "gitlab-ci.yml"       => '\u{f296}', // 
    "go.mod"              => '\u{e626}', // 
    "go.sum"              => '\u{e626}', // 
    "gradle"              => '\u{e256}', // 
    "gruntfile.coffee"    => '\u{e611}', // 
    "gruntfile.js"        => '\u{e611}', // 
    "gruntfile.ls"        => '\u{e611}', // 
    "gulpfile.coffee"     => '\u{e610}', // 
    "gulpfile.js"         => '\u{e610}', // 
    "gulpfile.ls"         => '\u{e610}', // 
    "hidden"              => '\u{f023}', // 
    "include"             => '\u{e5fc}', // 
    "lib"                 => '\u{f121}', // 
    "LICENSE"             => '\u{f02d}', // 
    "localized"           => '\u{f179}', // 
    "Makefile"            => '\u{f489}', // 
    "node_modules"        => '\u{e718}', // 
    "npmignore"           => '\u{e71e}', // 
    "PKGBUILD"            => '\u{f303}', // 
    "rubydoc"             => '\u{e73b}', // 
    "Vagrantfile"         => '\u{2371}', // ⍱
    "yarn.lock"           => '\u{e718}', // 
};

/// Mapping from lowercase file extension to icons.  If an image, video, or audio extension is add
/// also update the extension filetype map.
const EXTENSION_ICONS: Map<&'static str, char> = phf_map! {
    "7z"             => '\u{f410}',  // 
    "a"              => '\u{f17c}',  // 
    "acc"            => '\u{f001}',  // 
    "acf"            => '\u{f1b6}',  // 
    "ai"             => '\u{e7b4}',  // 
    "alac"           => '\u{f001}',  // 
    "android"        => '\u{e70e}',  // 
    "ape"            => '\u{f001}',  // 
    "apk"            => '\u{e70e}',  // 
    "apple"          => '\u{f179}',  // 
    "ar"             => '\u{f410}',  // 
    "arw"            => '\u{f1c5}',  // 
    "asm"            => '\u{e637}',  // 
    "avi"            => '\u{f03d}',  // 
    "avif"           => '\u{f1c5}',  // 
    "avro"           => '\u{e60b}',  // 
    "awk"            => '\u{f489}',  // 
    "bash"           => '\u{f489}',  // 
    "bashrc"         => '\u{f489}',  // 
    "bash_history"   => '\u{f489}',  // 
    "bash_profile"   => '\u{f489}',  // 
    "bat"            => '\u{ebc4}',  // 
    "bats"           => '\u{f489}',  // 
    "bib"            => '\u{e69b}',  // 
    "bin"            => '\u{eae8}',  // 
    "bmp"            => '\u{f1c5}',  // 
    "bst"            => '\u{e69b}',  // 
    "bz"             => '\u{f410}',  // 
    "bz2"            => '\u{f410}',  // 
    "c"              => '\u{e61e}',  // 
    "c++"            => '\u{e61d}',  // 
    "cab"            => '\u{e70f}',  // 
    "cbr"            => '\u{f1c5}',  // 
    "cbz"            => '\u{f1c5}',  // 
    "cc"             => '\u{e61d}',  // 
    "cert"           => '\u{eafa}',  // 
    "cfg"            => '\u{e615}',  // 
    "cjs"            => '\u{e74e}',  // 
    "class"          => '\u{e256}',  // 
    "clj"            => '\u{e768}',  // 
    "cljs"           => '\u{e76a}',  // 
    "cls"            => '\u{e69b}',  // 
    "cmd"            => '\u{e70f}',  // 
    "coffee"         => '\u{f0f4}',  // 
    "conf"           => '\u{e615}',  // 
    "config"         => '\u{e615}',  // 
    "cp"             => '\u{e61d}',  // 
    "cpio"           => '\u{f410}',  // 
    "cpp"            => '\u{e61d}',  // 
    "cr2"            => '\u{f1c5}',  // 
    "crt"            => '\u{eafa}',  // 
    "cs"             => '\u{f031b}', // 󰌛
    "csh"            => '\u{f489}',  // 
    "cshtml"         => '\u{f1fa}',  // 
    "csproj"         => '\u{f031b}', // 󰌛
    "css"            => '\u{e749}',  // 
    "csv"            => '\u{f1c3}',  // 
    "csx"            => '\u{f031b}', // 󰌛
    "cts"            => '\u{e628}',  // 
    "cu"             => '\u{e64b}',  // 
    "cxx"            => '\u{e61d}',  // 
    "d"              => '\u{e7af}',  // 
    "dart"           => '\u{e798}',  // 
    "db"             => '\u{f1c0}',  // 
    "deb"            => '\u{e77d}',  // 
    "desktop"        => '\u{ebd1}',  // 
    "diff"           => '\u{f440}',  // 
    "djvu"           => '\u{f02d}',  // 
    "dll"            => '\u{e70f}',  // 
    "dmg"            => '\u{e271}',  // 
    "doc"            => '\u{f1c2}',  // 
    "docx"           => '\u{f1c2}',  // 
    "drawio"         => '\u{ebba}',  // 
    "ds_store"       => '\u{f179}',  // 
    "dump"           => '\u{f1c0}',  // 
    "dvi"            => '\u{f1c5}',  // 
    "ebook"          => '\u{e28b}',  // 
    "ebuild"         => '\u{f30d}',  // 
    "editorconfig"   => '\u{e615}',  // 
    "ejs"            => '\u{e618}',  // 
    "el"             => '\u{e632}',  // 
    "elm"            => '\u{e62c}',  // 
    "eml"            => '\u{f003}',  // 
    "env"            => '\u{f462}',  // 
    "eot"            => '\u{f031}',  // 
    "eps"            => '\u{f1c5}',  // 
    "epub"           => '\u{e28a}',  // 
    "erb"            => '\u{e73b}',  // 
    "erl"            => '\u{e7b1}',  // 
    "ex"             => '\u{e62d}',  // 
    "exe"            => '\u{f17a}',  // 
    "exs"            => '\u{e62d}',  // 
    "fish"           => '\u{f489}',  // 
    "flac"           => '\u{f001}',  // 
    "flv"            => '\u{f03d}',  // 
    "font"           => '\u{f031}',  // 
    "fs"             => '\u{e7a7}',  // 
    "fsi"            => '\u{e7a7}',  // 
    "fsx"            => '\u{e7a7}',  // 
    "gdoc"           => '\u{f1c2}',  // 
    "gem"            => '\u{e21e}',  // 
    "gemfile"        => '\u{e21e}',  // 
    "gemspec"        => '\u{e21e}',  // 
    "gform"          => '\u{f298}',  // 
    "gif"            => '\u{f1c5}',  // 
    "git"            => '\u{f1d3}',  // 
    "gitattributes"  => '\u{f1d3}',  // 
    "gitignore"      => '\u{f1d3}',  // 
    "gitmodules"     => '\u{f1d3}',  // 
    "go"             => '\u{e626}',  // 
    "gpg"            => '\u{e60a}',  // 
    "gradle"         => '\u{e256}',  // 
    "groovy"         => '\u{e775}',  // 
    "gsheet"         => '\u{f1c3}',  // 
    "gslides"        => '\u{f1c4}',  // 
    "guardfile"      => '\u{e21e}',  // 
    "gz"             => '\u{f410}',  // 
    "h"              => '\u{f0fd}',  // 
    "hbs"            => '\u{e60f}',  // 
    "heic"           => '\u{f03d}',  // 
    "heif"           => '\u{f1c5}',  // 
    "hpp"            => '\u{f0fd}',  // 
    "hs"             => '\u{e777}',  // 
    "htm"            => '\u{f13b}',  // 
    "html"           => '\u{f13b}',  // 
    "hxx"            => '\u{f0fd}',  // 
    "ical"           => '\u{eab0}',  // 
    "icalendar"      => '\u{eab0}',  // 
    "ico"            => '\u{f1c5}',  // 
    "ics"            => '\u{eab0}',  // 
    "ifb"            => '\u{eab0}',  // 
    "image"          => '\u{f1c5}',  // 
    "img"            => '\u{e271}',  // 
    "iml"            => '\u{e7b5}',  // 
    "ini"            => '\u{f17a}',  // 
    "ipynb"          => '\u{e678}',  // 
    "iso"            => '\u{e271}',  // 
    "j2c"            => '\u{f1c5}',  // 
    "j2k"            => '\u{f1c5}',  // 
    "jad"            => '\u{e256}',  // 
    "jar"            => '\u{e256}',  // 
    "java"           => '\u{e256}',  // 
    "jfi"            => '\u{f1c5}',  // 
    "jfif"           => '\u{f1c5}',  // 
    "jif"            => '\u{f1c5}',  // 
    "jl"             => '\u{e624}',  // 
    "jmd"            => '\u{f48a}',  // 
    "jp2"            => '\u{f1c5}',  // 
    "jpe"            => '\u{f1c5}',  // 
    "jpeg"           => '\u{f1c5}',  // 
    "jpf"            => '\u{f1c5}',  // 
    "jpg"            => '\u{f1c5}',  // 
    "jpx"            => '\u{f1c5}',  // 
    "js"             => '\u{e74e}',  // 
    "json"           => '\u{e60b}',  // 
    "jsx"            => '\u{e7ba}',  // 
    "jxl"            => '\u{f1c5}',  // 
    "kdb"            => '\u{f23e}',  // 
    "kdbx"           => '\u{f23e}',  // 
    "key"            => '\u{eb11}',  // 
    "ko"             => '\u{f17c}',  // 
    "ksh"            => '\u{f489}',  // 
    "latex"          => '\u{e69b}',  // 
    "less"           => '\u{e758}',  // 
    "lhs"            => '\u{e777}',  // 
    "license"        => '\u{f02d}',  // 
    "localized"      => '\u{f179}',  // 
    "lock"           => '\u{f023}',  // 
    "log"            => '\u{f18d}',  // 
    "lua"            => '\u{e620}',  // 
    "lz"             => '\u{f410}',  // 
    "lz4"            => '\u{f410}',  // 
    "lzh"            => '\u{f410}',  // 
    "lzma"           => '\u{f410}',  // 
    "lzo"            => '\u{f410}',  // 
    "m"              => '\u{e61e}',  // 
    "m2ts"           => '\u{f03d}',  // 
    "m2v"            => '\u{f03d}',  // 
    "m4a"            => '\u{f001}',  // 
    "m4v"            => '\u{f03d}',  // 
    "magnet"         => '\u{f076}',  // 
    "markdown"       => '\u{f48a}',  // 
    "md"             => '\u{f48a}',  // 
    "mjs"            => '\u{e74e}',  // 
    "mk"             => '\u{f489}',  // 
    "mka"            => '\u{f001}',  // 
    "mkd"            => '\u{f48a}',  // 
    "mkv"            => '\u{f03d}',  // 
    "ml"             => '\u{e67a}',  // 
    "mli"            => '\u{e67a}',  // 
    "mll"            => '\u{e67a}',  // 
    "mly"            => '\u{e67a}',  // 
    "mm"             => '\u{e61d}',  // 
    "mobi"           => '\u{e28b}',  // 
    "mov"            => '\u{f03d}',  // 
    "mp2"            => '\u{f001}',  // 
    "mp3"            => '\u{f001}',  // 
    "mp4"            => '\u{f03d}',  // 
    "mpeg"           => '\u{f03d}',  // 
    "mpg"            => '\u{f03d}',  // 
    "msi"            => '\u{e70f}',  // 
    "mts"            => '\u{e628}',  // 
    "mustache"       => '\u{e60f}',  // 
    "nef"            => '\u{f1c5}',  // 
    "ninja"          => '\u{f0774}', // 󰝴
    "nix"            => '\u{f313}',  // 
    "node"           => '\u{f0399}', // 󰎙
    "npmignore"      => '\u{e71e}',  // 
    "o"              => '\u{eae8}',  // 
    "odp"            => '\u{f1c4}',  // 
    "ods"            => '\u{f1c3}',  // 
    "odt"            => '\u{f1c2}',  // 
    "ogg"            => '\u{f001}',  // 
    "ogm"            => '\u{f03d}',  // 
    "ogv"            => '\u{f03d}',  // 
    "opus"           => '\u{f001}',  // 
    "orf"            => '\u{f1c5}',  // 
    "org"            => '\u{e633}',  // 
    "otf"            => '\u{f031}',  // 
    "out"            => '\u{eb2c}',  // 
    "par"            => '\u{f410}',  // 
    "part"           => '\u{f43a}',  // 
    "patch"          => '\u{f440}',  // 
    "pbm"            => '\u{f1c5}',  // 
    "pdf"            => '\u{f1c1}',  // 
    "pem"            => '\u{eb11}',  // 
    "pgm"            => '\u{f1c5}',  // 
    "php"            => '\u{e73d}',  // 
    "pl"             => '\u{e769}',  // 
    "plx"            => '\u{e769}',  // 
    "pm"             => '\u{e769}',  // 
    "png"            => '\u{f1c5}',  // 
    "pnm"            => '\u{f1c5}',  // 
    "pod"            => '\u{e769}',  // 
    "ppm"            => '\u{f1c5}',  // 
    "ppt"            => '\u{f1c4}',  // 
    "pptx"           => '\u{f1c4}',  // 
    "procfile"       => '\u{e21e}',  // 
    "properties"     => '\u{e60b}',  // 
    "ps"             => '\u{f1c5}',  // 
    "ps1"            => '\u{ebc7}',  // 
    "psd"            => '\u{e7b8}',  // 
    "psd1"           => '\u{ebc7}',  // 
    "psm1"           => '\u{ebc7}',  // 
    "pxm"            => '\u{f1c5}',  // 
    "py"             => '\u{e606}',  // 
    "pyc"            => '\u{e606}',  // 
    "qcow2"          => '\u{e271}',  // 
    "r"              => '\u{f25d}',  // 
    "rakefile"       => '\u{e21e}',  // 
    "rar"            => '\u{f410}',  // 
    "raw"            => '\u{f1c5}',  // 
    "razor"          => '\u{f1fa}',  // 
    "rb"             => '\u{e21e}',  // 
    "rdata"          => '\u{f25d}',  // 
    "rdb"            => '\u{e76d}',  // 
    "rdoc"           => '\u{f48a}',  // 
    "rds"            => '\u{f25d}',  // 
    "readme"         => '\u{f48a}',  // 
    "rlib"           => '\u{e7a8}',  // 
    "rmd"            => '\u{f48a}',  // 
    "rmeta"          => '\u{e7a8}',  // 
    "rpm"            => '\u{e7bb}',  // 
    "rs"             => '\u{e7a8}',  // 
    "rspec"          => '\u{e21e}',  // 
    "rspec_parallel" => '\u{e21e}',  // 
    "rspec_status"   => '\u{e21e}',  // 
    "rss"            => '\u{f09e}',  // 
    "rst"            => '\u{f15c}',  // 
    "rtf"            => '\u{f0219}', // 󰈙
    "ru"             => '\u{e21e}',  // 
    "rubydoc"        => '\u{e73b}',  // 
    "s"              => '\u{e637}',  // 
    "sass"           => '\u{e603}',  // 
    "scala"          => '\u{e737}',  // 
    "scss"           => '\u{e749}',  // 
    "service"        => '\u{eba2}',  // 
    "sh"             => '\u{f489}',  // 
    "shell"          => '\u{f489}',  // 
    "slim"           => '\u{e73b}',  // 
    "sln"            => '\u{e70c}',  // 
    "so"             => '\u{f17c}',  // 
    "sql"            => '\u{f1c0}',  // 
    "sqlite3"        => '\u{e7c4}',  // 
    "stl"            => '\u{f1c5}',  // 
    "sty"            => '\u{e69b}',  // 
    "styl"           => '\u{e600}',  // 
    "stylus"         => '\u{e600}',  // 
    "svelte"         => '\u{e697}',  // 
    "svg"            => '\u{f1c5}',  // 
    "swift"          => '\u{e755}',  // 
    "t"              => '\u{e769}',  // 
    "tar"            => '\u{f410}',  // 
    "taz"            => '\u{f410}',  // 
    "tbz"            => '\u{f410}',  // 
    "tbz2"           => '\u{f410}',  // 
    "tc"             => '\u{f410}',  // 
    "tex"            => '\u{e69b}',  // 
    "tgz"            => '\u{f410}',  // 
    "tif"            => '\u{f1c5}',  // 
    "tiff"           => '\u{f1c5}',  // 
    "tlz"            => '\u{f410}',  // 
    "toml"           => '\u{e615}',  // 
    "torrent"        => '\u{e275}',  // 
    "ts"             => '\u{e628}',  // 
    "tsv"            => '\u{f1c3}',  // 
    "tsx"            => '\u{e7ba}',  // 
    "ttf"            => '\u{f031}',  // 
    "twig"           => '\u{e61c}',  // 
    "txt"            => '\u{f15c}',  // 
    "txz"            => '\u{f410}',  // 
    "tz"             => '\u{f410}',  // 
    "tzo"            => '\u{f410}',  // 
    "unity"          => '\u{e721}',  // 
    "unity3d"        => '\u{e721}',  // 
    "vdi"            => '\u{e271}',  // 
    "vhd"            => '\u{e271}',  // 
    "video"          => '\u{f03d}',  // 
    "vim"            => '\u{e7c5}',  // 
    "vmdk"           => '\u{e271}',  // 
    "vob"            => '\u{f03d}',  // 
    "vue"            => '\u{f0844}', // 󰡄
    "war"            => '\u{e256}',  // 
    "wav"            => '\u{f001}',  // 
    "webm"           => '\u{f03d}',  // 
    "webp"           => '\u{f1c5}',  // 
    "windows"        => '\u{f17a}',  // 
    "wma"            => '\u{f001}',  // 
    "wmv"            => '\u{f03d}',  // 
    "woff"           => '\u{f031}',  // 
    "woff2"          => '\u{f031}',  // 
    "xhtml"          => '\u{f13b}',  // 
    "xls"            => '\u{f1c3}',  // 
    "xlsm"           => '\u{f1c3}',  // 
    "xlsx"           => '\u{f1c3}',  // 
    "xml"            => '\u{f05c0}', // 󰗀
    "xpm"            => '\u{f1c5}',  // 
    "xul"            => '\u{f05c0}', // 󰗀
    "xz"             => '\u{f410}',  // 
    "yaml"           => '\u{f481}',  // 
    "yml"            => '\u{f481}',  // 
    "z"              => '\u{f410}',  // 
    "zig"            => '\u{21af}',  // ↯
    "zip"            => '\u{f410}',  // 
    "zsh"            => '\u{f489}',  // 
    "zsh-theme"      => '\u{f489}',  // 
    "zshrc"          => '\u{f489}',  // 
    "zst"            => '\u{f410}',  // 
};

/// Converts the style used to paint a file name into the style that should be
/// used to paint an icon.
///
/// - The background colour should be preferred to the foreground colour, as
///   if one is set, it’s the more “obvious” colour choice.
/// - If neither is set, just use the default style.
/// - Attributes such as bold or underline should not be used to paint the
///   icon, as they can make it look weird.
pub fn iconify_style(style: Style) -> Style {
    style.background.or(style.foreground)
         .map(Style::from)
         .unwrap_or_default()
}

pub fn icon_for_file(file: &File<'_>) -> char {
    if let Some(icon) = FILENAME_ICONS.get(file.name.as_str()) {
        *icon
    } else if file.points_to_directory() {
        if file.is_empty_dir() {
            '\u{f115}' // 
        } else {
            '\u{f07b}' // 
        }
    } else if let Some(ext) = file.ext.as_ref() {
        *EXTENSION_ICONS.get(ext.as_str()).unwrap_or(&'\u{f15b}') // 
    } else {
        '\u{f016}' // 
    }
}
