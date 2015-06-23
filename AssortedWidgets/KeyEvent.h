#pragma once

#include "Event.h"

namespace AssortedWidgets
{
	namespace Event
	{
		class KeyEvent : public Event
		{
		public:
			KeyEvent(Widgets::Component* _source, int _type, int _keyCode, int _modifiers):Event(_source,_type),keyCode(_keyCode),modifiers(_modifiers)
			{};

			enum KeyEventTypes
			{
				KEY_PRESSED,
				KEY_RELEASED,
				KEY_TYPED
			};

			enum VirtualKeys
			{
				VKUI_UNKNOWN		= 0,
				VKUI_FIRST			= 0,
				VKUI_BACKSPACE		= 8,
				VKUI_TAB			= 9,
				VKUI_ENTER			= 10,
				VKUI_CLEAR			= 12,
				VKUI_RETURN			= 13,
				VKUI_CTRL			= 17,
				VKUI_ALT			= 18,
				VKUI_PAUSE			= 19,
				VKUI_CAPSLOCK		= 20,
				VKUI_ESCAPE			= 27,
				VKUI_SPACE			= 32,
				VKUI_EXCLAIM			= 33,
				VKUI_QUOTEDBL			= 34,
				VKUI_HASH				= 35,
				VKUI_DOLLAR			= 36,
				VKUI_AMPERSAND		= 38,
				VKUI_QUOTE			= 39,
				VKUI_LEFTPAREN		= 40,
				VKUI_RIGHTPAREN		= 41,
				VKUI_ASTERISK			= 42,
				VKUI_PLUS				= 43,
				VKUI_COMMA			= 44,
				VKUI_MINUS			= 45,
				VKUI_PERIOD			= 46,
				VKUI_SLASH			= 47,
				VKUI_0				= 48,
				VKUI_1				= 49,
				VKUI_2				= 50,
				VKUI_3				= 51,
				VKUI_4				= 52,
				VKUI_5				= 53,
				VKUI_6				= 54,
				VKUI_7				= 55,
				VKUI_8				= 56,
				VKUI_9				= 57,
				VKUI_COLON			= 58,
				VKUI_SEMICOLON		= 59,
				VKUI_LESS				= 60,
				VKUI_EQUALS			= 61,
				VKUI_GREATER			= 62,
				VKUI_QUESTION			= 63,
				VKUI_AT				= 64,
				

				VKUI_LEFTBRACKET		= 91,
				VKUI_BACKSLASH		= 92,
				VKUI_RIGHTBRACKET		= 93,
				VKUI_CARET			= 94,
				VKUI_UNDERSCORE		= 95,
				VKUI_BACKQUOTE		= 96,
				VKUI_A				= 97,
				VKUI_B				= 98,
				VKUI_C				= 99,
				VKUI_D				= 100,
				VKUI_E				= 101,
				VKUI_F				= 102,
				VKUI_G				= 103,
				VKUI_H				= 104,
				VKUI_I				= 105,
				VKUI_J				= 106,
				VKUI_K				= 107,
				VKUI_L				= 108,
				VKUI_M				= 109,
				VKUI_N				= 110,
				VKUI_O				= 111,
				VKUI_P				= 112,
				VKUI_Q				= 113,
				VKUI_R				= 114,
				VKUI_S				= 115,
				VKUI_T				= 116,
				VKUI_U				= 117,
				VKUI_V				= 118,
				VKUI_W				= 119,
				VKUI_X				= 120,
				VKUI_Y				= 121,
				VKUI_Z				= 122,
				VKUI_DELETE			= 127,

				// numeric pad
				VKUI_KP0				= 256,
				VKUI_KP1				= 257,
				VKUI_KP2				= 258,
				VKUI_KP3				= 259,
				VKUI_KP4				= 260,
				VKUI_KP5				= 261,
				VKUI_KP6				= 262,
				VKUI_KP7				= 263,
				VKUI_KP8				= 264,
				VKUI_KP9				= 265,
				VKUI_KP_PERIOD		= 266,
				VKUI_KP_DIVIDE		= 267,
				VKUI_KP_MULTIPLY		= 268,
				VKUI_KP_MINUS			= 269,
				VKUI_KP_PLUS			= 270,
				VKUI_KP_ENTER			= 271,
				VKUI_KP_EQUALS		= 272,

				
				VKUI_UP				= 273,
				VKUI_DOWN				= 274,
				VKUI_RIGHT			= 275,
				VKUI_LEFT				= 276,
				VKUI_INSERT			= 277,
				VKUI_HOME				= 278,
				VKUI_END				= 279,
				VKUI_PAGEUP			= 280,
				VKUI_PAGEDOWN			= 281,

				VKUI_F1				= 282,
				VKUI_F2				= 283,
				VKUI_F3				= 284,
				VKUI_F4				= 285,
				VKUI_F5				= 286,
				VKUI_F6				= 287,
				VKUI_F7				= 288,
				VKUI_F8				= 289,
				VKUI_F9				= 290,
				VKUI_F10				= 291,
				VKUI_F11				= 292,
				VKUI_F12				= 293,
				VKUI_F13				= 294,
				VKUI_F14				= 295,
				VKUI_F15				= 296,

				VKUI_NUMLOCK			= 300,
				//VKUI_CAPSLOCK			= 301,
				VKUI_SCROLLOCK		= 302,
				VKUI_RSHIFT			= 303,
				VKUI_LSHIFT			= 304,
				VKUI_RCTRL			= 305,
				VKUI_LCTRL			= 306,
				VKUI_RALT				= 307,
				VKUI_LALT				= 308,
				VKUI_RMETA			= 309,
				VKUI_LMETA			= 310,
				VKUI_LSUPER			= 311,
				VKUI_RSUPER			= 312,
				VKUI_MODE			= 313,
				VKUI_COMPOSE		= 314,

				VKUI_HELP			= 315,
				VKUI_PRINT			= 316,
				VKUI_SYSREQ			= 317,
				VKUI_BREAK			= 318,
				VKUI_MENU			= 319,
				VKUI_EURO			= 321
			};

			enum Modifiers
			{
				MOD_NONE  = 0x0000,
				MOD_LSHIFT= 0x0001,
				MOD_RSHIFT= 0x0002,
				MOD_LCTRL = 0x0040,
				MOD_RCTRL = 0x0080,
				MOD_LALT  = 0x0100,
				MOD_RALT  = 0x0200,
				MOD_LMETA = 0x0400,
				MOD_RMETA = 0x0800,
				MOD_NUM   = 0x1000,
				MOD_CAPS  = 0x2000,
				MOD_MODE  = 0x4000,
				MOD_RESERVED = 0x8000
			};

			int getKeyCode() const
			{
				return keyCode;
			};

			int getModifier() const
			{
				return modifiers;
			};

		private:
			int keyCode, modifiers;
		};
	}
}