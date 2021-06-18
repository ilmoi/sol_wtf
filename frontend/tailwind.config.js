module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: 'class', // or 'media' or 'class'
  theme: {
    extend: {
      // add custom https://tailwindcss.com/docs/width
      width: {
        '550': '550px',
      },
      maxWidth: {
        '550': '550px',
      },
      screens: {
        'fivefifty': '550px',
        'fiveh': '500px',
        'fourh': '400px',
        'threeh': '300px',
      },
      colors: {
        solana: {
          verylightgreen: '#dcf2ea',
          lightgreen: '#7fe8c1',
          green: '#01f39d',
          darkgreen: '#015e3a',
          verydarkgreen: '#002b1b',

          verylightblue: '#d7e0ef',
          lightblue: '#91ade0',
          blue: '#3764BB',
          darkblue: '#274682',
          verydarkblue: '#1c325b',

          verylightpink: '#efceec',
          lightpink: '#e0a6da',
          pink: '#FE47ED',
          darkpink: '#b733aa',
          verydarkpink: '#601b59',

          verylightpurple: '#e5d5f2',
          lightpurple: '#c08fe8',
          purple: '#9c22fb',
          darkpurple: '#6e19b5',
          verydarkpurple: '#3b105e',
        },
      },
      textShadow: {
        'sol-lg-l': '5px 5px 0 #000',
        'sol-lg-d': '5px 5px 0 #fff',
        'sol-md-l': '4px 4px 0 #000',
        'sol-md-d': '4px 4px 0 #fff',
        'sol-sm-l': '2px 2px 0 #000',
        'sol-sm-d': '2px 2px 0 #fff',
      }
    },
  },
  variants: {
    extend: {
      backgroundColor: ['checked'],
      borderColor: ['checked'],
    }
  },
  plugins: [
    require('tailwindcss-textshadow')
  ],
}
