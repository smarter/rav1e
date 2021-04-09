pub const OC_COMP_BINS: usize = 24;
pub const OC_SATD_SHIFT: usize = 9;
pub const OC_LOGQ_BINS: usize = 8;

pub static OC_MODE_LOGQ: [[[i16; 2]; 3]; 8/*OC_LOGQ_BINS*/] =
  [
    [ [0x1F05,0x2101],[0x206E,0x2101],[0x206E,0x2101] ],
    [ [0x1C9A,0x1EAC],[0x1E0E,0x1EAC],[0x1E0E,0x1EAC] ],
    [ [0x1A31,0x1C48],[0x1B6F,0x1C48],[0x1B6F,0x1C48] ],
    [ [0x17B0,0x19E7],[0x1938,0x19E7],[0x1938,0x19E7] ],
    [ [0x152F,0x178F],[0x16AB,0x178F],[0x16AB,0x178F] ],
    [ [0x12F1,0x1534],[0x145D,0x1534],[0x145D,0x1534] ],
    [ [0x0FF3,0x1321],[0x11BE,0x1321],[0x11BE,0x1321] ],
    [ [0, 0], [0, 0], [0, 0] ]
  ];

// Last level is oc_mode_rd
pub const OC_MODE_RD_SATD: [[[[[i16; 2]; OC_COMP_BINS]; 2]; 3]; OC_LOGQ_BINS] = [
  [
    [
      /*Y'  qi=0  INTRA*/
      [
        [    0,    0],[    0,    0],[   11,   86],[    8,   80],
        [    2,  119],[    7,  145],[    1,  177],[    7,  194],
        [   12,  211],[    9,  230],[   20,  255],[   19,  266],
        [   20,  281],[   17,  297],[   15,  314],[   20,  333],
        [   22,  351],[   18,  333],[   30,  468],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    9,   52]
      ],
      /*Y'  qi=0  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=0  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=0  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=0  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=0  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=9  INTRA*/
      [
        [    0,    0],[    6,   52],[    4,   59],[    5,   87],
        [    7,  112],[    8,  133],[   10,  153],[   11,  171],
        [   12,  188],[   14,  204],[   15,  218],[   17,  233],
        [   19,  246],[   21,  260],[   23,  273],[   25,  284],
        [   27,  297],[   30,  308],[   32,  316],[   36,  334],
        [   38,  344],[   42,  353],[   44,  361],[   47,  369]
      ],
      /*Y'  qi=9  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=9  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=9  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=9  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=9  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=18  INTRA*/
      [
        [    0,    0],[    4,   32],[    5,   54],[    7,   79],
        [    9,   99],[   12,  117],[   14,  135],[   16,  149],
        [   20,  164],[   23,  175],[   27,  187],[   31,  198],
        [   35,  209],[   38,  220],[   42,  229],[   47,  239],
        [   51,  247],[   56,  255],[   61,  264],[   65,  267],
        [   71,  274],[   75,  280],[   80,  287],[   86,  293]
      ],
      /*Y'  qi=18  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=18  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=18  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=18  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=18  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=27  INTRA*/
      [
        [    9,   30],[    5,   26],[    6,   50],[   11,   69],
        [   15,   86],[   19,  102],[   24,  115],[   30,  126],
        [   36,  137],[   44,  145],[   51,  154],[   59,  161],
        [   66,  169],[   74,  177],[   82,  184],[   90,  191],
        [   99,  198],[  107,  203],[  116,  207],[  125,  213],
        [  133,  217],[  143,  221],[  151,  225],[  161,  228]
      ],
      /*Y'  qi=27  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=27  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=27  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=27  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=27  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=36  INTRA*/
      [
        [   10,   -5],[    3,   25],[    9,   44],[   15,   60],
        [   24,   75],[   33,   86],[   45,   96],[   57,  103],
        [   69,  111],[   82,  116],[   94,  122],[  108,  128],
        [  121,  133],[  135,  138],[  150,  143],[  163,  147],
        [  178,  151],[  191,  154],[  205,  157],[  217,  159],
        [  231,  161],[  243,  163],[  256,  166],[  269,  167]
      ],
      /*Y'  qi=36  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=36  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=36  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=36  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=36  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=45  INTRA*/
      [
        [   -6,   -2],[    8,   23],[   17,   39],[   30,   53],
        [   44,   64],[   62,   73],[   81,   79],[   99,   85],
        [  120,   90],[  138,   94],[  158,   98],[  177,  101],
        [  197,  104],[  218,  107],[  239,  110],[  258,  113],
        [  279,  114],[  296,  116],[  313,  117],[  328,  118],
        [  345,  119],[  358,  120],[  376,  121],[  390,  122]
      ],
      /*Y'  qi=45  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=45  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=45  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=45  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=45  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=54  INTRA*/
      [
        [    7,    0],[    4,   21],[   11,   34],[   41,   43],
        [   78,   49],[  114,   53],[  149,   57],[  182,   59],
        [  216,   62],[  243,   63],[  273,   64],[  301,   66],
        [  330,   67],[  361,   68],[  389,   69],[  415,   69],
        [  440,   70],[  461,   70],[  481,   71],[  499,   71],
        [  519,   71],[  537,   71],[  556,   71],[  572,   71]
      ],
      /*Y'  qi=54  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=54  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=54  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=54  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=54  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ],
  [
    [
      /*Y'  qi=63  INTRA*/
      [
        [  -36,    8],[   54,   13],[  302,    7],[  528,   -7],
        [  679,  -15],[  811,  -21],[  909,  -28],[  992,  -31],
        [ 1076,  -37],[ 1138,  -38],[ 1201,  -40],[ 1262,  -44],
        [ 1316,  -46],[ 1369,  -48],[ 1415,  -50],[ 1454,  -50],
        [ 1496,  -52],[ 1530,  -52],[ 1556,  -54],[ 1586,  -54],
        [ 1614,  -54],[ 1631,  -54],[ 1665,  -54],[ 1684,  -53]
      ],
      /*Y'  qi=63  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cb  qi=63  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cb  qi=63  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ],
    [
      /*Cr  qi=63  INTRA*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ],
      /*Cr  qi=63  INTER*/
      [
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0],
        [    0,    0],[    0,    0],[    0,    0],[    0,    0]
      ]
    ]
  ]
];
