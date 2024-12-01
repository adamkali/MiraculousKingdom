
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const MKConfig: CustomThemeConfig = {
    name: 'mk-skeleton-config',
    properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "var(--color-primary-100)",
		"--theme-rounded-base": "16px",
		"--theme-rounded-container": "8px",
		"--theme-border-base": "2px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "0 0 0",
		"--on-secondary": "0 0 0",
		"--on-tertiary": "var(--color-tertiary-200)",
		"--on-success": "255 255 255",
		"--on-warning": "0 0 0",
		"--on-error": "0 0 0",
		"--on-surface": "var(--color-primary-50)",
		// =~= Theme Colors  =~=
		// primary | #0dd1aa 
		"--color-primary-50": "219 248 242", // #dbf8f2
		"--color-primary-100": "207 246 238", // #cff6ee
		"--color-primary-200": "195 244 234", // #c3f4ea
		"--color-primary-300": "158 237 221", // #9eeddd
		"--color-primary-400": "86 223 196", // #56dfc4
		"--color-primary-500": "13 209 170", // #0dd1aa
		"--color-primary-600": "12 188 153", // #0cbc99
		"--color-primary-700": "10 157 128", // #0a9d80
		"--color-primary-800": "8 125 102", // #087d66
		"--color-primary-900": "6 102 83", // #066653
		// secondary | #cd33de 
		"--color-secondary-50": "248 224 250", // #f8e0fa
		"--color-secondary-100": "245 214 248", // #f5d6f8
		"--color-secondary-200": "243 204 247", // #f3ccf7
		"--color-secondary-300": "235 173 242", // #ebadf2
		"--color-secondary-400": "220 112 232", // #dc70e8
		"--color-secondary-500": "205 51 222", // #cd33de
		"--color-secondary-600": "185 46 200", // #b92ec8
		"--color-secondary-700": "154 38 167", // #9a26a7
		"--color-secondary-800": "123 31 133", // #7b1f85
		"--color-secondary-900": "100 25 109", // #64196d
		// tertiary | #2c22e3 
		"--color-tertiary-50": "223 222 251", // #dfdefb
		"--color-tertiary-100": "213 211 249", // #d5d3f9
		"--color-tertiary-200": "202 200 248", // #cac8f8
		"--color-tertiary-300": "171 167 244", // #aba7f4
		"--color-tertiary-400": "107 100 235", // #6b64eb
		"--color-tertiary-500": "44 34 227", // #2c22e3
		"--color-tertiary-600": "40 31 204", // #281fcc
		"--color-tertiary-700": "33 26 170", // #211aaa
		"--color-tertiary-800": "26 20 136", // #1a1488
		"--color-tertiary-900": "22 17 111", // #16116f
		// success | #8b51a3 
		"--color-success-50": "238 229 241", // #eee5f1
		"--color-success-100": "232 220 237", // #e8dced
		"--color-success-200": "226 212 232", // #e2d4e8
		"--color-success-300": "209 185 218", // #d1b9da
		"--color-success-400": "174 133 191", // #ae85bf
		"--color-success-500": "139 81 163", // #8b51a3
		"--color-success-600": "125 73 147", // #7d4993
		"--color-success-700": "104 61 122", // #683d7a
		"--color-success-800": "83 49 98", // #533162
		"--color-success-900": "68 40 80", // #442850
		// warning | #ede103 
		"--color-warning-50": "252 251 217", // #fcfbd9
		"--color-warning-100": "251 249 205", // #fbf9cd
		"--color-warning-200": "251 248 192", // #fbf8c0
		"--color-warning-300": "248 243 154", // #f8f39a
		"--color-warning-400": "242 234 79", // #f2ea4f
		"--color-warning-500": "237 225 3", // #ede103
		"--color-warning-600": "213 203 3", // #d5cb03
		"--color-warning-700": "178 169 2", // #b2a902
		"--color-warning-800": "142 135 2", // #8e8702
		"--color-warning-900": "116 110 1", // #746e01
		// error | #dda217 
		"--color-error-50": "250 241 220", // #faf1dc
		"--color-error-100": "248 236 209", // #f8ecd1
		"--color-error-200": "247 232 197", // #f7e8c5
		"--color-error-300": "241 218 162", // #f1daa2
		"--color-error-400": "231 190 93", // #e7be5d
		"--color-error-500": "221 162 23", // #dda217
		"--color-error-600": "199 146 21", // #c79215
		"--color-error-700": "166 122 17", // #a67a11
		"--color-error-800": "133 97 14", // #85610e
		"--color-error-900": "108 79 11", // #6c4f0b
		// surface | #001a4a 
		"--color-surface-50": "217 221 228", // #d9dde4
		"--color-surface-100": "204 209 219", // #ccd1db
		"--color-surface-200": "191 198 210", // #bfc6d2
		"--color-surface-300": "153 163 183", // #99a3b7
		"--color-surface-400": "77 95 128", // #4d5f80
		"--color-surface-500": "0 26 74", // #001a4a
		"--color-surface-600": "0 23 67", // #001743
		"--color-surface-700": "0 20 56", // #001438
		"--color-surface-800": "0 16 44", // #00102c
		"--color-surface-900": "0 13 36", // #000d24
		
	}
}
