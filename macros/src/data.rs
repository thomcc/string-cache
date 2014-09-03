// Copyright 2014 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub static atoms: &'static [&'static str] = &[

    // The first 64 atoms are special: we can quickly check membership
    // in sets of these, using a bitmask.  This includes every tag that
    // appears in more than one set in the tree builder spec, plus a
    // few others (arbitrarily chosen).
    //
    // FIXME(kmc): check if this is really true with the packed tag bits

    "a",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "b",
    "base",
    "basefont",
    "bgsound",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "caption",
    "col",
    "colgroup",
    "dd",
    "dt",
    "embed",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "html",
    "input",
    "li",
    "link",
    "marquee",
    "meta",
    "noframes",
    "noscript",
    "object",
    "optgroup",
    "option",
    "param",
    "plaintext",
    "pre",
    "rp",
    "rt",
    "script",
    "select",
    "source",
    "style",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "xmp",

    // End of first 64 atoms.

    "",
    "abbr",
    "abs",
    "accent",
    "accent-height",
    "accentunder",
    "accept",
    "accept-charset",
    "accesskey",
    "accumulate",
    "acronym",
    "action",
    "actiontype",
    "active",
    "actuate",
    "additive",
    "align",
    "alignment-baseline",
    "alignmentscope",
    "alink",
    "alphabetic",
    "alt",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "altglyph",
    "altglyphdef",
    "altglyphitem",
    "altimg",
    "alttext",
    "amplitude",
    "and",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "animatecolor",
    "animatemotion",
    "animatetransform",
    "animation",
    "annotation",
    "annotation-xml",
    "apply",
    "approx",
    "arabic-form",
    "arccos",
    "arccosh",
    "arccot",
    "arccoth",
    "arccsc",
    "arccsch",
    "archive",
    "arcrole",
    "arcsec",
    "arcsech",
    "arcsin",
    "arcsinh",
    "arctan",
    "arctanh",
    "arg",
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-channel",
    "aria-checked",
    "aria-controls",
    "aria-datatype",
    "aria-describedby",
    "aria-disabled",
    "aria-dropeffect",
    "aria-expanded",
    "aria-flowto",
    "aria-grab",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-multiline",
    "aria-multiselectable",
    "aria-owns",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-secret",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-templateid",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "ascent",
    "async",
    "attributeName",
    "attributeType",
    "attributename",
    "attributetype",
    "audio",
    "autocomplete",
    "autofocus",
    "autoplay",
    "autosubmit",
    "axis",
    "azimuth",
    "background",
    "baseFrequency",
    "baseProfile",
    "basefrequency",
    "baseline",
    "baseline-shift",
    "baseprofile",
    "bbox",
    "bdo",
    "begin",
    "bevelled",
    "bgcolor",
    "bias",
    "border",
    "bvar",
    "by",
    "calcMode",
    "calcmode",
    "canvas",
    "cap-height",
    "card",
    "cartesianproduct",
    "ceiling",
    "cellpadding",
    "cellspacing",
    "center",
    "char",
    "charoff",
    "charset",
    "checked",
    "ci",
    "circle",
    "cite",
    "class",
    "classid",
    "clear",
    "clip",
    "clip-path",
    "clip-rule",
    "clipPath",
    "clipPathUnits",
    "clippath",
    "clippathunits",
    "close",
    "closure",
    "cn",
    "code",
    "codebase",
    "codetype",
    "codomain",
    "color",
    "color-interpolation",
    "color-interpolation-filters",
    "color-profile",
    "color-rendering",
    "cols",
    "colspan",
    "columnalign",
    "columnlines",
    "columnspacing",
    "columnspan",
    "columnwidth",
    "compact",
    "complexes",
    "compose",
    "condition",
    "conjugate",
    "content",
    "contentScriptType",
    "contentStyleType",
    "contenteditable",
    "contentscripttype",
    "contentstyletype",
    "contextmenu",
    "controls",
    "coords",
    "cos",
    "cosh",
    "cot",
    "coth",
    "crossorigin",
    "csc",
    "csch",
    "csymbol",
    "curl",
    "cursor",
    "cx",
    "cy",
    "d",
    "data",
    "datafld",
    "dataformatas",
    "datasrc",
    "datatemplate",
    "datetime",
    "declare",
    "default",
    "defer",
    "definition-src",
    "definitionURL",
    "definitionurl",
    "defs",
    "degree",
    "del",
    "depth",
    "desc",
    "descent",
    "details",
    "determinant",
    "dfn",
    "dialog",
    "diff",
    "diffuseConstant",
    "diffuseconstant",
    "dir",
    "direction",
    "disabled",
    "discard",
    "display",
    "displaystyle",
    "div",
    "divergence",
    "divide",
    "divisor",
    "dl",
    "domain",
    "domainofapplication",
    "dominant-baseline",
    "draggable",
    "dur",
    "dx",
    "dy",
    "edge",
    "edgeMode",
    "edgemode",
    "elevation",
    "ellipse",
    "em",
    "emptyset",
    "enable-background",
    "encoding",
    "enctype",
    "end",
    "eq",
    "equalcolumns",
    "equalrows",
    "equivalent",
    "eulergamma",
    "exists",
    "exp",
    "exponent",
    "exponentiale",
    "externalResourcesRequired",
    "externalresourcesrequired",
    "face",
    "factorial",
    "factorof",
    "false",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "feblend",
    "fecolormatrix",
    "fecomponenttransfer",
    "fecomposite",
    "feconvolvematrix",
    "fediffuselighting",
    "fedisplacementmap",
    "fedistantlight",
    "feflood",
    "fefunca",
    "fefuncb",
    "fefuncg",
    "fefuncr",
    "fegaussianblur",
    "feimage",
    "femerge",
    "femergenode",
    "femorphology",
    "fence",
    "feoffset",
    "fepointlight",
    "fespecularlighting",
    "fespotlight",
    "fetile",
    "feturbulence",
    "fieldset",
    "figcaption",
    "figure",
    "fill",
    "fill-opacity",
    "fill-rule",
    "filter",
    "filterRes",
    "filterUnits",
    "filterres",
    "filterunits",
    "flood-color",
    "flood-opacity",
    "floor",
    "fn",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "font-family",
    "font-size",
    "font-size-adjust",
    "font-stretch",
    "font-style",
    "font-variant",
    "font-weight",
    "fontfamily",
    "fontsize",
    "fontstyle",
    "fontweight",
    "footer",
    "for",
    "forall",
    "foreignObject",
    "foreignobject",
    "format",
    "frameborder",
    "framespacing",
    "from",
    "fx",
    "fy",
    "g",
    "g1",
    "g2",
    "gcd",
    "geq",
    "glyph",
    "glyph-name",
    "glyph-orientation-horizontal",
    "glyph-orientation-vertical",
    "glyphRef",
    "glyphref",
    "grad",
    "gradientTransform",
    "gradientUnits",
    "gradienttransform",
    "gradientunits",
    "groupalign",
    "gt",
    "handler",
    "hanging",
    "header",
    "headers",
    "height",
    "hgroup",
    "hidden",
    "hidefocus",
    "high",
    "hkern",
    "horiz-adv-x",
    "horiz-origin-x",
    "horiz-origin-y",
    "hr",
    "href",
    "hreflang",
    "hspace",
    "http-equiv",
    "i",
    "icon",
    "id",
    "ident",
    "ideographic",
    "iframe",
    "image",
    "image-rendering",
    "imaginary",
    "imaginaryi",
    "img",
    "implies",
    "in",
    "in2",
    "index",
    "infinity",
    "inputmode",
    "ins",
    "int",
    "integers",
    "intercept",
    "intersect",
    "interval",
    "inverse",
    "irrelevant",
    "isindex",
    "ismap",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kbd",
    "kernelMatrix",
    "kernelUnitLength",
    "kernelmatrix",
    "kernelunitlength",
    "kerning",
    "keyPoints",
    "keySplines",
    "keyTimes",
    "keygen",
    "keypoints",
    "keysplines",
    "keytimes",
    "label",
    "lambda",
    "lang",
    "language",
    "laplacian",
    "largeop",
    "lcm",
    "legend",
    "lengthAdjust",
    "lengthadjust",
    "leq",
    "letter-spacing",
    "lighting-color",
    "limit",
    "limitingConeAngle",
    "limitingconeangle",
    "line",
    "linearGradient",
    "lineargradient",
    "linebreak",
    "linethickness",
    "list",
    "listener",
    "listing",
    "ln",
    "local",
    "log",
    "logbase",
    "longdesc",
    "loop",
    "low",
    "lowlimit",
    "lowsrc",
    "lquote",
    "lspace",
    "lt",
    "macros",
    "maction",
    "main",
    "maligngroup",
    "malignmark",
    "manifest",
    "map",
    "marginheight",
    "marginwidth",
    "mark",
    "marker",
    "marker-end",
    "marker-mid",
    "marker-start",
    "markerHeight",
    "markerUnits",
    "markerWidth",
    "markerheight",
    "markerunits",
    "markerwidth",
    "mask",
    "maskContentUnits",
    "maskUnits",
    "maskcontentunits",
    "maskunits",
    "math",
    "mathbackground",
    "mathcolor",
    "mathematical",
    "mathsize",
    "mathvariant",
    "matrix",
    "matrixrow",
    "max",
    "maxlength",
    "maxsize",
    "mean",
    "media",
    "median",
    "mediummathspace",
    "menclose",
    "menu",
    "menuitem",
    "merror",
    "metadata",
    "meter",
    "method",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "min",
    "minsize",
    "minus",
    "missing-glyph",
    "mlabeledtr",
    "mmultiscripts",
    "mn",
    "mo",
    "mode",
    "moment",
    "momentabout",
    "movablelimits",
    "mover",
    "mpadded",
    "mpath",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "multiple",
    "munder",
    "munderover",
    "name",
    "nargs",
    "naturalnumbers",
    "nav",
    "neq",
    "nest",
    "nobr",
    "noembed",
    "nohref",
    "none",
    "noresize",
    "noshade",
    "not",
    "notanumber",
    "notation",
    "notin",
    "notprsubset",
    "notsubset",
    "nowrap",
    "numOctaves",
    "numoctaves",
    "occurrence",
    "offset",
    "ol",
    "onabort",
    "onactivate",
    "onafterprint",
    "onafterupdate",
    "onbefordeactivate",
    "onbeforeactivate",
    "onbeforecopy",
    "onbeforecut",
    "onbeforeeditfocus",
    "onbeforepaste",
    "onbeforeprint",
    "onbeforeunload",
    "onbeforeupdate",
    "onbegin",
    "onblur",
    "onbounce",
    "oncellchange",
    "onchange",
    "onclick",
    "oncontextmenu",
    "oncontrolselect",
    "oncopy",
    "oncut",
    "ondataavailable",
    "ondatasetchanged",
    "ondatasetcomplete",
    "ondblclick",
    "ondeactivate",
    "ondrag",
    "ondragdrop",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "onend",
    "onerror",
    "onerrorupdate",
    "onfilterchange",
    "onfinish",
    "onfocus",
    "onfocusin",
    "onfocusout",
    "onformchange",
    "onforminput",
    "onhelp",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onload",
    "onlosecapture",
    "onmessage",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onmousewheel",
    "onmove",
    "onmoveend",
    "onmovestart",
    "onpaste",
    "onpropertychange",
    "onreadystatechange",
    "onrepeat",
    "onreset",
    "onresize",
    "onrowenter",
    "onrowexit",
    "onrowsdelete",
    "onrowsinserted",
    "onscroll",
    "onselect",
    "onselectstart",
    "onstart",
    "onstop",
    "onsubmit",
    "onunload",
    "onzoom",
    "opacity",
    "open",
    "operator",
    "optimum",
    "or",
    "order",
    "orient",
    "orientation",
    "origin",
    "other",
    "otherwise",
    "outerproduct",
    "output",
    "overflow",
    "overline-position",
    "overline-thickness",
    "p",
    "panose-1",
    "partialdiff",
    "path",
    "pathLength",
    "pathlength",
    "pattern",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "patterncontentunits",
    "patterntransform",
    "patternunits",
    "pi",
    "piece",
    "piecewise",
    "ping",
    "plus",
    "pointer-events",
    "points",
    "pointsAtX",
    "pointsAtY",
    "pointsAtZ",
    "pointsatx",
    "pointsaty",
    "pointsatz",
    "polygon",
    "polyline",
    "poster",
    "power",
    "prefetch",
    "preserveAlpha",
    "preserveAspectRatio",
    "preservealpha",
    "preserveaspectratio",
    "primes",
    "primitiveUnits",
    "primitiveunits",
    "product",
    "profile",
    "progress",
    "prompt",
    "prsubset",
    "q",
    "quotient",
    "r",
    "radialGradient",
    "radialgradient",
    "radiogroup",
    "radius",
    "rationals",
    "readonly",
    "real",
    "reals",
    "rect",
    "refX",
    "refY",
    "refx",
    "refy",
    "rel",
    "reln",
    "rem",
    "rendering-intent",
    "repeat",
    "repeat-max",
    "repeat-min",
    "repeat-start",
    "repeat-template",
    "repeatCount",
    "repeatDur",
    "repeatcount",
    "repeatdur",
    "replace",
    "required",
    "requiredExtensions",
    "requiredFeatures",
    "requiredextensions",
    "requiredfeatures",
    "restart",
    "result",
    "rev",
    "role",
    "root",
    "rotate",
    "rowalign",
    "rowlines",
    "rows",
    "rowspacing",
    "rowspan",
    "rquote",
    "rspace",
    "ruby",
    "rule",
    "rules",
    "rx",
    "ry",
    "s",
    "samp",
    "sandbox",
    "scalarproduct",
    "scale",
    "scheme",
    "scope",
    "scoped",
    "scriptlevel",
    "scriptminsize",
    "scriptsizemultiplier",
    "scrolldelay",
    "scrolling",
    "sdev",
    "seamless",
    "sec",
    "sech",
    "section",
    "seed",
    "selected",
    "selection",
    "selector",
    "semantics",
    "sep",
    "separator",
    "separators",
    "set",
    "setdiff",
    "shape",
    "shape-rendering",
    "show",
    "sin",
    "sinh",
    "size",
    "slope",
    "small",
    "solidcolor",
    "space",
    "spacing",
    "span",
    "specification",
    "specularConstant",
    "specularExponent",
    "specularconstant",
    "specularexponent",
    "speed",
    "spreadMethod",
    "spreadmethod",
    "src",
    "srcdoc",
    "standby",
    "start",
    "startOffset",
    "startoffset",
    "stdDeviation",
    "stddeviation",
    "stemh",
    "stemv",
    "step",
    "stitchTiles",
    "stitchtiles",
    "stop",
    "stop-color",
    "stop-opacity",
    "stretchy",
    "strike",
    "strikethrough-position",
    "strikethrough-thickness",
    "string",
    "stroke",
    "stroke-dasharray",
    "stroke-dashoffset",
    "stroke-linecap",
    "stroke-linejoin",
    "stroke-miterlimit",
    "stroke-opacity",
    "stroke-width",
    "strong",
    "sub",
    "subscriptshift",
    "subset",
    "sum",
    "summary",
    "sup",
    "superscriptshift",
    "surfaceScale",
    "surfacescale",
    "switch",
    "symbol",
    "symmetric",
    "systemLanguage",
    "systemlanguage",
    "tabindex",
    "tableValues",
    "tablevalues",
    "tan",
    "tanh",
    "target",
    "targetX",
    "targetY",
    "targetx",
    "targety",
    "tbreak",
    "tendsto",
    "text",
    "text-anchor",
    "text-decoration",
    "text-rendering",
    "textLength",
    "textPath",
    "textlength",
    "textpath",
    "thickmathspace",
    "thinmathspace",
    "time",
    "times",
    "to",
    "transform",
    "transpose",
    "tref",
    "true",
    "tspan",
    "tt",
    "type",
    "u",
    "u1",
    "u2",
    "ul",
    "underline-position",
    "underline-thickness",
    "unicode",
    "unicode-bidi",
    "unicode-range",
    "union",
    "units-per-em",
    "unselectable",
    "uplimit",
    "use",
    "usemap",
    "v-alphabetic",
    "v-hanging",
    "v-ideographic",
    "v-mathematical",
    "valign",
    "value",
    "values",
    "valuetype",
    "var",
    "variance",
    "vector",
    "vectorproduct",
    "version",
    "vert-adv-y",
    "vert-origin-x",
    "vert-origin-y",
    "verythickmathspace",
    "verythinmathspace",
    "veryverythickmathspace",
    "veryverythinmathspace",
    "video",
    "view",
    "viewBox",
    "viewTarget",
    "viewbox",
    "viewtarget",
    "visibility",
    "vkern",
    "vlink",
    "vspace",
    "wbr",
    "when",
    "width",
    "widths",
    "word-spacing",
    "wrap",
    "writing-mode",
    "x",
    "x-height",
    "x1",
    "x2",
    "xChannelSelector",
    "xchannelselector",
    "xlink:actuate",
    "xlink:arcrole",
    "xlink:href",
    "xlink:role",
    "xlink:show",
    "xlink:type",
    "xml:base",
    "xml:lang",
    "xml:space",
    "xmlns",
    "xmlns:xlink",
    "xor",
    "xref",
    "y",
    "y1",
    "y2",
    "yChannelSelector",
    "ychannelselector",
    "z",
    "zoomAndPan",
    "zoomandpan",
];
