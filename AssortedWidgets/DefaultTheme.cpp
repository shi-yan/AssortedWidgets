#include "DefaultTheme.h"
#include "SDL2/SDL.h"
#include "SDL2/SDL_opengl.h"
#include "SDL2/SDL_image.h"
#include "FontEngine.h"
#include "Menu.h"
#include "MenuBar.h"
#include "MenuItemButton.h"
#include "MenuList.h"
#include "MenuItemSeparator.h"
#include "Graphics.h"
#include "MenuItemSubMenu.h"
#include "Label.h"
#include "Button.h"
#include "MenuItemToggleButton.h"
#include "MenuItemRadioButton.h"
#include "MenuItemRadioGroup.h"
#include "Dialog.h"
#include "DialogTittleBar.h"
#include "TextField.h"
#include "Logo.h"
#include "ScrollBarButton.h"
#include "ScrollBarSlider.h"
#include "ScrollBar.h"
#include "ScrollPanel.h"
#include "CheckButton.h"
#include "RadioButton.h"
#include "ProgressBar.h"
#include "SlideBarSlider.h"
#include "SlideBar.h"
#include "DropListButton.h"
#include "DropList.h"
#include "DropListItem.h"

namespace AssortedWidgets
{
	namespace Theme
	{
		DefaultTheme::DefaultTheme(unsigned int _width,unsigned int _height)
		{
			screenWidth=_width;
			screenHeight=_height;
		}

		void DefaultTheme::test()
		{
			glEnable(GL_TEXTURE_2D);
			glColor3ub(255,255,255);
			glBindTexture(GL_TEXTURE_2D,textureID);
			glBegin(GL_QUADS);
			glTexCoord2f(0.0f,0.0f);
			glVertex3f(0,0,0);
			glTexCoord2f(1.0f,0.0f);
			glVertex3f(256,0,0);
			glTexCoord2f(1.0f,1.0f);
			glVertex3f(256,256,0);
			glTexCoord2f(0.0f,1.0f);
			glVertex3f(0,256,0);
			glEnd();
			glDisable(GL_TEXTURE_2D);
		}

		void DefaultTheme::setup()
		{
            SDL_RWops *io = SDL_RWFromFile("aw.png", "r+b");
            SDL_Surface *img=IMG_LoadPNG_RW(io);
		    SDL_LockSurface(img);   
			glEnable(GL_TEXTURE_2D);
			glGenTextures(1,&textureID);
			glBindTexture(GL_TEXTURE_2D,textureID);
			glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, img->w, img->h, 0, GL_RGBA, GL_UNSIGNED_BYTE, img->pixels);
			
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,GL_NEAREST);
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);

			glDisable(GL_TEXTURE_2D);
			SDL_UnlockSurface(img);
			SDL_FreeSurface(img);

			MenuLeft=new SubImage(1.0/256.0,1.0/256.0,7.0/256.0,21.0/256.0,textureID);
			MenuRight=new SubImage(53.0/256.0,1.0/256.0,59.0/256.0,21.0/256.0,textureID);

			MenuListUpLeft=new SubImage(3.0/256.0,34.0/256.0,27.0/256.0,43.0/256.0,textureID);
			MenuListUp=new SubImage(22.0/256.0,34.0/256.0,31.0/256.0,43.0/256.0,textureID);
			MenuListUpRight=new SubImage(33.0/256.0,34.0/256.0,57.0/256.0,43.0/256.0,textureID);
			MenuListLeft=new SubImage(3.0/256.0,38.0/256.0,27.0/256.0,43.0/256.0,textureID);
			MenuListRight=new SubImage(33.0/256.0,38.0/256.0,57.0/256.0,43.0/256.0,textureID);
			MenuListBottomLeft=new SubImage(3.0/256.0,44.0/256.0,27.0/256.0,60.0/256.0,textureID);
			MenuListBottom=new SubImage(22.0/256.0,44.0/256.0,31.0/256.0,60.0/256.0,textureID);
			MenuListBottomRight=new SubImage(33.0/256.0,44.0/256.0,57.0/256.0,60.0/256.0,textureID);
			MenuItemSubMenuArrow=new SubImage(62.0/256.0,1.0/256.0,67.0/256.0,10.0/256.0,textureID);
			ButtonNormalLeft=new SubImage(1.0/256.0,61.0/256.0,5.0/256.0,80.0/256.0,textureID);
			ButtonNormalRight=new SubImage(83.0/256.0,61.0/256.0,87.0/256.0,80.0/256.0,textureID);
			ButtonHoverLeft=new SubImage(1.0/256.0,81.0/256.0,5.0/256.0,100.0/256.0,textureID);
			ButtonHoverRight=new SubImage(83.0/256.0,81.0/256.0,87.0/256.0,100.0/256.0,textureID);
			RightHook=new SubImage(69.0/256.0,1.0/256.0,77.0/256.0,10.0/256.0,textureID);
			RadioDot=new SubImage(78.0/256.0,1.0/256.0,87.0/256.0,10.0/256.0,textureID);
			DialogUpLeftActive=new SubImage(3.0/256.0,27.0/256.0,27.0/256.0,43.0/256.0,textureID);
			DialogUpActive=new SubImage(22.0/256.0,27.0/256.0,31.0/256.0,43.0/256.0,textureID);
			DialogUpRightActive=new SubImage(33.0/256.0,27.0/256.0,57.0/256.0,43.0/256.0,textureID);
			DialogLeft=new SubImage(3.0/256.0,38.0/256.0,27.0/256.0,43.0/256.0,textureID);
			DialogRight=new SubImage(33.0/256.0,38.0/256.0,57.0/256.0,43.0/256.0,textureID);
			DialogBottomLeft=new SubImage(3.0/256.0,44.0/256.0,27.0/256.0,60.0/256.0,textureID);
			DialogBottom=new SubImage(22.0/256.0,44.0/256.0,31.0/256.0,60.0/256.0,textureID);
			DialogBottomRight=new SubImage(33.0/256.0,44.0/256.0,57.0/256.0,60.0/256.0,textureID);
			TextFieldLeft=new SubImage(1.0/256.0,101.0/256.0,5.0/256.0,121.0/256.0,textureID);
			TextFieldRight=new SubImage(47.0/256.0,101.0/256.0,51.0/256.0,121.0/256.0,textureID);
			Logo=new SubImage(0.0/256.0,169.0/256.0,253.0/256.0,255.0/256.0,textureID);

			ScrollBarVerticalTopNormal=new SubImage(1.0/256.0,122.0/256.0,16.0/256.0,137.0/256.0,textureID);
			ScrollBarVerticalBottomNormal=new SubImage(33.0/256.0,122.0/256.0,48.0/256.0,137.0/256.0,textureID);
			ScrollBarHorizontalLeftNormal=new SubImage(17.0/256.0,122.0/256.0,32.0/256.0,137.0/256.0,textureID);
			ScrollBarHorizontalRightNormal=new SubImage(49.0/256.0,122.0/256.0,64.0/256.0,137.0/256.0,textureID);

			ScrollBarVerticalTopHover=new SubImage(81.0/256.0,101.0/256.0,96.0/256.0,116.0/256.0,textureID);
			ScrollBarVerticalBottomHover=new SubImage(113.0/256.0,101.0/256.0,128.0/256.0,116.0/256.0,textureID);
			ScrollBarHorizontalLeftHover=new SubImage(97.0/256.0,101.0/256.0,112.0/256.0,116.0/256.0,textureID);
			ScrollBarHorizontalRightHover=new SubImage(129.0/256.0,101.0/256.0,143.0/256.0,116.0/256.0,textureID);

			ScrollBarHorizontalBackground=new SubImage(2.0/256.0,138.0/256.0,12.0/256.0,153.0/256.0,textureID);
			ScrollBarVerticalBackground=new SubImage(65.0/256.0,102.0/256.0,80.0/256.0,112.0/256.0,textureID);


			CheckButtonOn=new SubImage(81.0/256.0,129.0/256.0,92.0/256.0,140.0/256.0,textureID);
			CheckButtonOff=new SubImage(94.0/256.0,129.0/256.0,105.0/256.0,140.0/256.0,textureID);
			RadioButtonOn=new SubImage(81.0/256.0,117.0/256.0,92.0/256.0,128.0/256.0,textureID);
			RadioButtonOff=new SubImage(94.0/256.0,117.0/256.0,105.0/256.0,128.0/256.0,textureID);

			ProgressBarLeft=new SubImage(1.0/256.0,101.0/256.0,5.0/256.0,121.0/256.0,textureID);
			ProgressBarRight=new SubImage(47.0/256.0,101.0/256.0,51.0/256.0,121.0/256.0,textureID);
			ProgressBarTop=new SubImage(106.0/256.0,117.0/256.0,126.0/256.0,121.0/256.0,textureID);
			ProgressBarBottom=new SubImage(106.0/256.0,145.0/256.0,126.0/256.0,149.0/256.0,textureID);

			DialogUpLeftDeactive=new SubImage(89.0/256.0,61.0/256.0,113.0/256.0,77.0/256.0,textureID);
			DialogUpDeactive=new SubImage(111.0/256.0,61.0/256.0,116.0/256.0,77.0/256.0,textureID);
			DialogUpRightDeactive=new SubImage(119.0/256.0,61.0/256.0,143.0/256.0,77.0/256.0,textureID);
		};

		Util::Size DefaultTheme::getMenuPreferedSize(Widgets::Menu *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
			return Util::Size(12+text.width,19);
		};

		void DefaultTheme::paintMenu(Widgets::Menu *component)
		{
			if(component->isExpand())
			{
				float x1=static_cast<float>(component->position.x);
				float y1=static_cast<float>(component->position.y);
				float x2=static_cast<float>(component->position.x+component->size.width);
				float y2=static_cast<float>(component->position.y+component->size.height);
				glDisable(GL_TEXTURE_2D);
				glColor3ub(44,55,55);
				glBegin(GL_QUADS);
				glVertex2f(x1+6,y1);
				glVertex2f(x2-6,y1);
				glVertex2f(x2-6,y2);
				glVertex2f(x1+6,y2);
				glEnd();
				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				MenuLeft->paint(x1,y1,x1+6,y2);
				MenuRight->paint(x2-6,y1,x2,y2);
				glColor3ub(150,155,161);
				Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
			}
			else
			{
				switch(component->getStatus())
				{
					case Widgets::Menu::hover :
					{
						float x1=static_cast<float>(component->position.x);
						float y1=static_cast<float>(component->position.y);
						float x2=static_cast<float>(component->position.x+component->size.width);
						float y2=static_cast<float>(component->position.y+component->size.height);
						glDisable(GL_TEXTURE_2D);
						glColor3ub(44,55,55);
						glBegin(GL_QUADS);
						glVertex2f(x1+6,y1);
						glVertex2f(x2-6,y1);
						glVertex2f(x2-6,y2);
						glVertex2f(x1+6,y2);
						glEnd();
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						MenuLeft->paint(x1,y1,x1+6,y2);
						MenuRight->paint(x2-6,y1,x2,y2);
						glColor3ub(150,155,161);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
					case Widgets::Menu::normal:
					{
						float x1=static_cast<float>(component->position.x);
						float y1=static_cast<float>(component->position.y);
						glColor3ub(150,155,161);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
					case Widgets::Menu::pressed:
					{
						float x1=static_cast<float>(component->position.x);
						float y1=static_cast<float>(component->position.y);
						glColor3ub(250,250,250);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
				}
			}
		};

		Util::Size DefaultTheme::getMenuBarPreferedSize(Widgets::MenuBar *component)
		{
			return Util::Size(component->size.width,30);
		};

		void DefaultTheme::paintMenuBar(Widgets::MenuBar *component)
		{
			float x1=0.0f;
			float y1=0.0f;
			float x2=static_cast<float>(component->size.width);
			float y2=30.0f;
			glDisable(GL_TEXTURE_2D);
			glBegin(GL_QUADS);
			glColor3ub(42,55,55);
			glVertex2f(x1,y1);
			glVertex2f(x1+40.0f,y1);
			glVertex2f(x1+40.0f,y2);
			glVertex2f(x1,y2);
			glColor3ub(55,65,67);
			glVertex2f(x1+40.0f,y1);
			glVertex2f(x2,y1);
			glVertex2f(x2,y2);
			glVertex2f(x1+40.0f,y2);
			glEnd();
		};

		Util::Size DefaultTheme::getMenuListPreferedSize(Widgets::MenuList *component)
		{
			unsigned int miniSize(component->getMinimizeSize());
			unsigned int width(0);
			unsigned int height(0);

			std::vector<Widgets::MenuItem *>::iterator iter;
			for(iter=component->getItemList().begin();iter<component->getItemList().end();++iter)
			{
				Util::Size itemSize=(*iter)->getPreferedSize();
				width=std::max<unsigned int>(width,itemSize.width);
				height+=itemSize.height;
			}

			return Util::Size(width+component->getLeft()+component->getRight(),height+component->getTop()+component->getBottom());
		};

		void DefaultTheme::paintMenuList(Widgets::MenuList *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			float x1=static_cast<float>(origin.x+component->position.x);
			float y1=static_cast<float>(origin.y+component->position.y);
			float x2=static_cast<float>(x1+component->size.width);
			float y2=static_cast<float>(y1+component->size.height);
			glEnable(GL_TEXTURE_2D);
			glColor3ub(255,255,255);
			MenuListUpLeft->paint(x1,y1,x1+24.0f,y1+9.0f);
			MenuListUpRight->paint(x2-24.0f,y1,x2,y1+9.0f);
			MenuListUp->paint(x1+24.0f,y1,x2-24.0f,y1+9.0f);
			MenuListLeft->paint(x1,y1+9.0f,x1+24.0f,y2-16.0f);
			MenuListRight->paint(x2-24.0f,y1+9.0f,x2,y2-16.0f);
			MenuListBottomLeft->paint(x1,y2-16.0f,x1+24.0f,y2);
			MenuListBottomRight->paint(x2-24.0f,y2-16.0f,x2,y2);
			MenuListBottom->paint(x1+24.0f,y2-16.0f,x2-24.0f,y2);
			glDisable(GL_TEXTURE_2D);
			glColor3ub(46,55,53);
			glBegin(GL_QUADS);
			glVertex2f(x1+24.0f,y1+9.0f);
			glVertex2f(x2-24.0f,y1+9.0f);
			glVertex2f(x2-24.0f,y2-16.0f);
			glVertex2f(x1+24.0f,y2-16.0f);
			glEnd();
		};

		Util::Size DefaultTheme::getMenuItemButtonPreferedSize(Widgets::MenuItemButton *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
			return Util::Size(24+text.width,20);
		};

		Util::Size DefaultTheme::getMenuItemSeparatorPreferedSize(Widgets::MenuItemSeparator *component)
		{
			return Util::Size(component->size);
		};

		void DefaultTheme::paintMenuItemSeparator(Widgets::MenuItemSeparator *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			glDisable(GL_TEXTURE_2D);
			glColor3ub(79,91,84);
			glBegin(GL_LINES);
			glVertex2f(static_cast<float>(10+origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+1));
			glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-10),static_cast<float>(origin.y+component->position.y+1));
			glEnd();
		};

		void DefaultTheme::paintMenuItemButton(Widgets::MenuItemButton *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			switch(component->getStatus())
			{
				case Widgets::MenuItemButton::normal:
				{
					glColor3ub(255,255,255);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					break;
				};
				case Widgets::MenuItemButton::pressed:
				{
					glColor3ub(200,200,200);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					break;
				};
				case Widgets::MenuItemButton::hover:
				{
					glDisable(GL_TEXTURE_2D);
					glColor3ub(176,200,28);
					glBegin(GL_QUADS);
					glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
					glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y+component->size.height));
					glEnd();
					glColor3ub(88,101,9);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					break;
				};
			}
		}

		Util::Size DefaultTheme::getMenuItemSubMenuPreferedSize(Widgets::MenuItemSubMenu *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
			return Util::Size(24+10+text.width,20);
		}
			
		void DefaultTheme::paintMenuItemSubMenu(Widgets::MenuItemSubMenu *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			switch(component->getStatus())
			{
				case Widgets::MenuItemSubMenu::normal:
				{
					glColor3ub(255,255,255);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					if(component->isExpand())
					{
						MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->position.x+component->size.width-17),static_cast<float>(component->position.y+origin.y+5),static_cast<float>(origin.x+component->position.x+component->size.width-12),static_cast<float>(component->position.y+origin.y+14));
					}
					else
					{
						MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->position.x+component->size.width-22),static_cast<float>(component->position.y+origin.y+5),static_cast<float>(origin.x+component->position.x+component->size.width-17),static_cast<float>(component->position.y+origin.y+14));
					}
					break;
				};
				case Widgets::MenuItemSubMenu::pressed:
				{
					glColor3ub(200,200,200);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					break;
				};
				case Widgets::MenuItemSubMenu::hover:
				{
					glDisable(GL_TEXTURE_2D);
					glColor3ub(176,200,28);
					glBegin(GL_QUADS);
					glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
					glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y+component->size.height));
					glEnd();
					glColor3ub(88,101,9);
					Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					if(component->isExpand())
					{
						MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->position.x+component->size.width-17),static_cast<float>(component->position.y+origin.y+5),static_cast<float>(origin.x+component->position.x+component->size.width-12),static_cast<float>(component->position.y+origin.y+14));
					}
					else
					{
						MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->position.x+component->size.width-22),static_cast<float>(component->position.y+origin.y+5),static_cast<float>(origin.x+component->position.x+component->size.width-17),static_cast<float>(component->position.y+origin.y+14));
					}
					break;
				};
			}
		}

					Util::Size DefaultTheme::getLabelPreferedSize(Widgets::Label *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(component->getRight()+component->getLeft()+text.width,20);
			};

			void DefaultTheme::paintLabel(Widgets::Label *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->isDrawBackground())
				{
					glColor3ub(0,0,0);
					glBegin(GL_QUADS);
					glVertex2f(static_cast<GLfloat>(origin.x+component->position.x),static_cast<GLfloat>(origin.y+component->position.y));
					glVertex2f(static_cast<GLfloat>(origin.x+component->position.x+component->size.width),static_cast<GLfloat>(origin.y+component->position.y));
					glVertex2f(static_cast<GLfloat>(origin.x+component->position.x+component->size.width),static_cast<GLfloat>(origin.y+component->position.y+component->size.height));
					glVertex2f(static_cast<GLfloat>(origin.x+component->position.x),static_cast<GLfloat>(origin.y+component->position.y+component->size.height));
					glEnd();
				}
				glColor3ub(255,255,255);
				Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),component->getText());
			};

			Util::Size DefaultTheme::getButtonPreferedSize(Widgets::Button *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(component->getRight()+component->getLeft()+text.width,19);
			};
			
			void DefaultTheme::paintButton(Widgets::Button *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::Button::normal:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(137,155,145);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),component->getText());
						break;
					};

					case Widgets::Button::hover:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonHoverLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonHoverRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(175,200,28);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),component->getText());
						break;
					};

					case Widgets::Button::pressed:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),component->getText());
						break;
					};
				}
			};

			Util::Size DefaultTheme::getMenuItemToggleButtonPreferedSize(Widgets::MenuItemToggleButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(10+24+text.width,20);
			};

			void DefaultTheme::paintMenuItemToggleButton(Widgets::MenuItemToggleButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::MenuItemToggleButton::normal:
					{
						glColor3ub(255,255,255);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RightHook->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
					case Widgets::MenuItemToggleButton::pressed:
					{
						glColor3ub(200,200,200);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RightHook->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
					case Widgets::MenuItemToggleButton::hover:
					{
						glDisable(GL_TEXTURE_2D);
						glColor3ub(176,200,28);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
						glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y+component->size.height));
						glEnd();
						glColor3ub(88,101,9);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RightHook->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
				}
			};

			Util::Size DefaultTheme::getMenuItemRadioButtonPreferedSize(Widgets::MenuItemRadioButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(10+24+text.width,20);
			};

			void DefaultTheme::paintMenuItemRadioButton(Widgets::MenuItemRadioButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::MenuItemRadioButton::normal:
					{
						glColor3ub(255,255,255);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RadioDot->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
					case Widgets::MenuItemRadioButton::pressed:
					{
						glColor3ub(200,200,200);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RadioDot->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
					case Widgets::MenuItemRadioButton::hover:
					{
						glDisable(GL_TEXTURE_2D);
						glColor3ub(176,200,28);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(component->position.x+origin.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
						glVertex2f(static_cast<float>(component->position.x+origin.x),static_cast<float>(origin.y+component->position.y+component->size.height));
						glEnd();
						glColor3ub(88,101,9);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
						if(component->getToggle())
						{
							glEnable(GL_TEXTURE_2D);
							RadioDot->paint(static_cast<float>(component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y),static_cast<float>(8+component->position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->position.y+9));
						}
						break;
					};
				}
			};
			
			Util::Size DefaultTheme::getMenuItemRadioGroupPreferedSize(Widgets::MenuItemRadioGroup *component)
			{
				unsigned int miniSize(component->getMinimizeSize());
				unsigned int width(0);
				unsigned int height(0);

				std::vector<Widgets::MenuItemRadioButton *>::iterator iter;
				for(iter=component->getItemList().begin();iter<component->getItemList().end();++iter)
				{
					Util::Size itemSize=(*iter)->getPreferedSize();
					width=std::max<unsigned int>(width,itemSize.width);
					height+=itemSize.height;
				}
				return Util::Size(width+component->getLeft()+component->getRight(),height+component->getTop()+component->getBottom());
			};
			
			void DefaultTheme::paintMenuItemRadioGroup(Widgets::MenuItemRadioGroup *component)
			{
			};

			Util::Size DefaultTheme::getDialogPreferedSize(Widgets::Dialog *component)
			{
				return component->size;
			};

			void DefaultTheme::paintDialog(Widgets::Dialog *component)
			{
				float x1=static_cast<float>(component->position.x+24);
				float x2=static_cast<float>(component->position.x+component->size.width-24);
				float y1=static_cast<float>(component->position.y+component->size.height-16);
				float y2=static_cast<float>(component->position.y+component->size.height);

				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);

				if(component->isActive())
				{
					DialogUpLeftActive->paint(static_cast<float>(component->position.x),static_cast<float>(component->position.y),static_cast<float>(x1),static_cast<float>(component->position.y+16));
					DialogUpActive->paint(static_cast<float>(x1),static_cast<float>(component->position.y),static_cast<float>(x2),static_cast<float>(component->position.y+16));
					DialogUpRightActive->paint(static_cast<float>(x2),static_cast<float>(component->position.y),static_cast<float>(component->position.x+component->size.width),static_cast<float>(component->position.y+16));
				}
				else
				{
					DialogUpLeftDeactive->paint(static_cast<float>(component->position.x),static_cast<float>(component->position.y),static_cast<float>(x1),static_cast<float>(component->position.y+16));
					DialogUpDeactive->paint(static_cast<float>(x1),static_cast<float>(component->position.y),static_cast<float>(x2),static_cast<float>(component->position.y+16));
					DialogUpRightDeactive->paint(static_cast<float>(x2),static_cast<float>(component->position.y),static_cast<float>(component->position.x+component->size.width),static_cast<float>(component->position.y+16));
				}

				DialogLeft->paint(static_cast<float>(component->position.x),static_cast<float>(component->position.y+16),static_cast<float>(x1),static_cast<float>(y1));
				DialogRight->paint(static_cast<float>(x2),static_cast<float>(component->position.y+16),static_cast<float>(component->position.x+component->size.width),static_cast<float>(y1));
				DialogBottomLeft->paint(static_cast<float>(component->position.x),static_cast<float>(y1),static_cast<float>(x1),static_cast<float>(y2));
				DialogBottom->paint(static_cast<float>(x1),static_cast<float>(y1),static_cast<float>(x2),static_cast<float>(y2));
				DialogBottomRight->paint(static_cast<float>(x2),static_cast<float>(y1),static_cast<float>(component->position.x+component->size.width),static_cast<float>(y2));
				glDisable(GL_TEXTURE_2D);
				glColor3ub(46,55,53);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(x1),static_cast<float>(component->position.y+16));
				glVertex2f(static_cast<float>(x2),static_cast<float>(component->position.y+16));
				glVertex2f(static_cast<float>(x2),static_cast<float>(y1));
				glVertex2f(static_cast<float>(x1),static_cast<float>(y1));
				glEnd();
			};

			Util::Size DefaultTheme::getDialogTittleBarPreferedSize(Widgets::DialogTittleBar *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(20+text.width,20);
			};
			
			void DefaultTheme::paintDialogTittleBar(Widgets::DialogTittleBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glColor3ub(31,31,31);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+component->size.height));
				glEnd();
				glColor3ub(255,255,255);
				Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->position.y),component->getText());
			};

			Util::Size DefaultTheme::getTextFieldPreferedSize(Widgets::TextField *component)
			{
				return Util::Size(component->getLength()+12,20);
			};

			void DefaultTheme::paintTextField(Widgets::TextField *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				float x1=static_cast<float>(origin.x+component->position.x);
				float x2=static_cast<float>(origin.x+component->position.x+4);
				float x3=static_cast<float>(origin.x+component->position.x+component->size.width-4);
				float x4=static_cast<float>(origin.x+component->position.x+component->size.width);
				float y1=static_cast<float>(origin.y+component->position.y);
				float y2=static_cast<float>(origin.y+component->position.y+component->size.height);

				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				TextFieldLeft->paint(x1,y1,x2,y2);
				TextFieldRight->paint(x3,y1,x4,y2);
				glDisable(GL_TEXTURE_2D);
				glColor3ub(79,91,84);
				glBegin(GL_QUADS);
				glVertex2f(x2,y1);
				glVertex2f(x3,y1);
				glVertex2f(x3,y2);
				glVertex2f(x2,y2);
				glEnd();
				glColor3ub(0,0,0);
				if(component->isActive())
				{
					glBegin(GL_QUADS);
					glVertex2f(x3+2,y1+4);
					glVertex2f(x3+3,y1+4);
					glVertex2f(x3+3,y2-4);
					glVertex2f(x3+2,y2-4);
					glEnd();
				}
				glEnable(GL_TEXTURE_2D);
				Util::Size textSize=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				glEnable(GL_SCISSOR_TEST);
				glScissor(static_cast<GLint>(x1),static_cast<GLint>(screenHeight-y2),static_cast<GLint>(component->size.width),static_cast<GLint>(component->size.height));
				Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x3-4-textSize.width),static_cast<int>(component->getTop()+y1),component->getText());
				glDisable(GL_SCISSOR_TEST);
			};

			Util::Size DefaultTheme::getLogoPreferedSize(Widgets::Logo *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintLogo(Widgets::Logo *component)
			{
				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				Logo->paint(static_cast<float>(component->position.x),static_cast<float>(component->position.y),static_cast<float>(component->position.x+component->size.width),static_cast<float>(component->position.y+component->size.height));
			};

			Util::Size DefaultTheme::getScrollBarButtonPreferedSize(Widgets::ScrollBarButton *component)
			{
				return Util::Size();
			};
			void DefaultTheme::paintScrollBarButton(Widgets::ScrollBarButton *component)
			{
				SubImage *button=0;
				switch(component->getType())
				{
					case Widgets::ScrollBarButton::HorizontalLeft:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
							button=ScrollBarHorizontalLeftNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
							button=ScrollBarHorizontalLeftHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
							button=ScrollBarHorizontalLeftNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::HorizontalRight:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
							button=ScrollBarHorizontalRightNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
							button=ScrollBarHorizontalRightHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
							button=ScrollBarHorizontalRightNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::VerticalTop:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
							button=ScrollBarVerticalTopNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
							button=ScrollBarVerticalTopHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
							button=ScrollBarVerticalTopNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::VerticalBottom:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
							button=ScrollBarVerticalBottomNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
							button=ScrollBarVerticalBottomHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
							button=ScrollBarVerticalBottomNormal;
						}
						break;
					}
				}
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				button->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
			};

			Util::Size DefaultTheme::getScrollBarSliderPreferedSize(Widgets::ScrollBarSlider *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintScrollBarSlider(Widgets::ScrollBarSlider *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(46,55,53);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+component->size.height));
				glEnd();
			};

			Util::Size DefaultTheme::getScrollBarPreferedSize(Widgets::ScrollBar *component)
			{
				return Util::Size();
			};
						
			void DefaultTheme::paintScrollBar(Widgets::ScrollBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				if(component->getType()==Widgets::ScrollBar::Horizontal)
				{
					ScrollBarHorizontalBackground->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				}
				else if(component->getType()==Widgets::ScrollBar::Vertical)
				{
					ScrollBarVerticalBackground->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				}		
			};

			Util::Size DefaultTheme::getScrollPanelPreferedSize(Widgets::ScrollPanel *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintScrollPanel(Widgets::ScrollPanel *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(79,91,84);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+component->size.height));
				glEnd();

				if(component->isHorizontalBarShow()||component->isVerticalBarShow())
				{
					glColor3ub(46,55,53);
					glBegin(GL_QUADS);
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-15),static_cast<float>(origin.y+component->position.y+component->size.height-15));
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-1),static_cast<float>(origin.y+component->position.y+component->size.height-15));
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-1),static_cast<float>(origin.y+component->position.y+component->size.height-1));
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-15),static_cast<float>(origin.y+component->position.y+component->size.height-1));
					glEnd();
				}
			};

			void DefaultTheme::scissorBegin(Util::Position &position,Util::Size &area)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				/*glColor3ub(0,0,255);
				glBegin(GL_QUADS);
				glVertex2f(origin.x+position.x,origin.y+position.y);
				glVertex2f(origin.x+position.x+area.width,origin.y+position.y);
				glVertex2f(origin.x+position.x+area.width,origin.y+position.y+area.height);
				glVertex2f(origin.x+position.x,origin.y+position.y+area.height);
				glEnd();*/
				glEnable(GL_SCISSOR_TEST);
				glScissor(origin.x+position.x,screenHeight-origin.y-area.height-position.y,area.width,area.height);

			};

			void DefaultTheme::scissorEnd()
			{
				glDisable(GL_SCISSOR_TEST);
			};

			Util::Size DefaultTheme::getCheckButtonPreferedSize(Widgets::CheckButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(component->getRight()+component->getLeft()+text.width+15,19);
			};

			void DefaultTheme::paintCheckButton(Widgets::CheckButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				SubImage *checkStatus(0);
				if(component->isCheck())
				{
					checkStatus=CheckButtonOn;
				}
				else
				{
					checkStatus=CheckButtonOff;
				}
				switch(component->getStatus())
				{
					case Widgets::CheckButton::normal:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(137,155,145);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::hover:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonHoverLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonHoverRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(175,200,28);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::pressed:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};
				}
			};

			Util::Size DefaultTheme::getRadioButtonPreferedSize(Widgets::RadioButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(component->getRight()+component->getLeft()+text.width+15,19);
			};

			void DefaultTheme::paintRadioButton(Widgets::RadioButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				SubImage *checkStatus(0);
				if(component->isCheck())
				{
					checkStatus=RadioButtonOn;
				}
				else
				{
					checkStatus=RadioButtonOff;
				}
				switch(component->getStatus())
				{
					case Widgets::CheckButton::normal:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(137,155,145);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::hover:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonHoverLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonHoverRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(175,200,28);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::pressed:
					{
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						ButtonNormalLeft->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						ButtonNormalRight->paint(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+19));
						glDisable(GL_TEXTURE_2D);
						glColor3ub(55,67,65);
						glBegin(GL_QUADS);
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y));
						glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width-4),static_cast<float>(origin.y+component->position.y+19));
						glVertex2f(static_cast<float>(origin.x+component->position.x+4),static_cast<float>(origin.y+component->position.y+19));
						glEnd();
						glColor3ub(0,0,0);
						Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft()+15,origin.y+component->position.y+component->getTop(),component->getText());
						glEnable(GL_TEXTURE_2D);
						glColor3ub(255,255,255);
						checkStatus->paint(static_cast<float>(origin.x+component->position.x+component->getLeft()),static_cast<float>(origin.y+component->position.y+component->getTop()),static_cast<float>(origin.x+component->position.x+component->getLeft()+11),static_cast<float>(origin.y+component->position.y+component->getTop()+11));
						break;
					};
				}
			};

			Util::Size DefaultTheme::getProgressBarPreferedSize(Widgets::ProgressBar *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintProgressBar(Widgets::ProgressBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getType()==Widgets::ProgressBar::Horizontal)
				{
					Util::Position origin=Util::Graphics::getSingleton().getOrigin();
					float x1=static_cast<float>(origin.x+component->position.x);
					float x2=static_cast<float>(origin.x+component->position.x+4);
					float x3=static_cast<float>(origin.x+component->position.x+component->size.width-4);
					float x4=static_cast<float>(origin.x+component->position.x+component->size.width);
					float y1=static_cast<float>(origin.y+component->position.y);
					float y2=static_cast<float>(origin.y+component->position.y+component->size.height);

					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					ProgressBarLeft->paint(x1,y1,x2,y2);
					ProgressBarRight->paint(x3,y1,x4,y2);
					glDisable(GL_TEXTURE_2D);
					glColor3ub(79,91,84);
					glBegin(GL_QUADS);
					glVertex2f(x2,y1);
					glVertex2f(x3,y1);
					glVertex2f(x3,y2);
					glVertex2f(x2,y2);
					glEnd();

					glColor3ub(46,55,53);
					glBegin(GL_QUADS);
					glVertex2f(x1+2,y1+2);
					glVertex2f(x1+2+component->getPOfSlider(),y1+2);
					glVertex2f(x1+2+component->getPOfSlider(),y2-2);
					glVertex2f(x1+2,y2-2);
					glEnd();
				}
				else if(component->getType()==Widgets::ProgressBar::Vertical)
				{
					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					float x1=static_cast<float>(origin.x+component->position.x);
					float x2=static_cast<float>(origin.x+component->position.x+component->size.width);
					float y1=static_cast<float>(origin.y+component->position.y);
					float y2=static_cast<float>(origin.y+component->position.y+4);
					float y3=static_cast<float>(origin.y+component->position.y+component->size.height-4);
					float y4=static_cast<float>(origin.y+component->position.y+component->size.height);

					ProgressBarTop->paint(x1,y1,x2,y2);
					ProgressBarBottom->paint(x1,y3,x2,y4);
					glDisable(GL_TEXTURE_2D);
					glColor3ub(79,91,84);
					glBegin(GL_QUADS);
					glVertex2f(x1,y2);
					glVertex2f(x2,y2);
					glVertex2f(x2,y3);
					glVertex2f(x1,y3);
					glEnd();
					glColor3ub(46,55,53);
					glBegin(GL_QUADS);
					glVertex2f(x1+2,y4-2-component->getPOfSlider());
					glVertex2f(x2-2,y4-2-component->getPOfSlider());
					glVertex2f(x2-2,y4-2);
					glVertex2f(x1+2,y4-2);
					glEnd();
				}
			};

			Util::Size DefaultTheme::getSlideBarSliderPreferedSize(Widgets::SlideBarSlider *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintSlideBarSlider(Widgets::SlideBarSlider *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glDisable(GL_TEXTURE_2D);
				glColor3ub(46,55,53);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y));
				glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
				glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+component->size.height));
				glEnd();
			};

			Util::Size DefaultTheme::getSlideBarPreferedSize(Widgets::SlideBar *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintSlideBar(Widgets::SlideBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getType()==Widgets::SlideBar::Horizontal)
				{
					Util::Position origin=Util::Graphics::getSingleton().getOrigin();
					float x1=static_cast<float>(origin.x+component->position.x);
					float x2=static_cast<float>(origin.x+component->position.x+4);
					float x3=static_cast<float>(origin.x+component->position.x+component->size.width-4);
					float x4=static_cast<float>(origin.x+component->position.x+component->size.width);
					float y1=static_cast<float>(origin.y+component->position.y);
					float y2=static_cast<float>(origin.y+component->position.y+component->size.height);

					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					ProgressBarLeft->paint(x1,y1,x2,y2);
					ProgressBarRight->paint(x3,y1,x4,y2);
					glDisable(GL_TEXTURE_2D);
					glColor3ub(79,91,84);
					glBegin(GL_QUADS);
					glVertex2f(x2,y1);
					glVertex2f(x3,y1);
					glVertex2f(x3,y2);
					glVertex2f(x2,y2);
					glEnd();
				}
				else if(component->getType()==Widgets::SlideBar::Vertical)
				{
					glEnable(GL_TEXTURE_2D);
					glColor3ub(255,255,255);
					float x1=static_cast<float>(origin.x+component->position.x);
					float x2=static_cast<float>(origin.x+component->position.x+component->size.width);
					float y1=static_cast<float>(origin.y+component->position.y);
					float y2=static_cast<float>(origin.y+component->position.y+4);
					float y3=static_cast<float>(origin.y+component->position.y+component->size.height-4);
					float y4=static_cast<float>(origin.y+component->position.y+component->size.height);

					ProgressBarTop->paint(x1,y1,x2,y2);
					ProgressBarBottom->paint(x1,y3,x2,y4);
					glDisable(GL_TEXTURE_2D);
					glColor3ub(79,91,84);
					glBegin(GL_QUADS);
					glVertex2f(x1,y2);
					glVertex2f(x2,y2);
					glVertex2f(x2,y3);
					glVertex2f(x1,y3);
					glEnd();
				}
			};

			Util::Size DefaultTheme::getDropListButtonPreferedSize(Widgets::DropListButton *component)
			{
				return Util::Size();
			}

			void DefaultTheme::paintDropListButton(Widgets::DropListButton *component)
			{
				SubImage *button(0);
				switch(component->getStatus())
				{
					case Widgets::DropListButton::normal:
					{
						button=ScrollBarVerticalBottomNormal;
						break;
					}
					case Widgets::DropListButton::hover:
					{
						button=ScrollBarVerticalBottomHover;
						break;
					}
					case Widgets::DropListButton::pressed:
					{
						button=ScrollBarVerticalBottomNormal;
						break;
					}
				}
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				button->paint(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y),static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
			};

			Util::Size DefaultTheme::getDropListPreferedSize(Widgets::DropList *component)
			{
				return Util::Size();
			};

			void DefaultTheme::paintDropList(Widgets::DropList *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				float x1=static_cast<float>(origin.x+component->position.x);
				float x2=static_cast<float>(origin.x+component->position.x+4);
				float x3=static_cast<float>(origin.x+component->position.x+component->size.width-4);
				float x4=static_cast<float>(origin.x+component->position.x+component->size.width);
				float y1=static_cast<float>(origin.y+component->position.y);
				float y2=static_cast<float>(origin.y+component->position.y+component->size.height);

				glEnable(GL_TEXTURE_2D);
				glColor3ub(255,255,255);
				ProgressBarLeft->paint(x1,y1,x2,y2);
				ProgressBarRight->paint(x3,y1,x4,y2);
				glDisable(GL_TEXTURE_2D);
				glColor3ub(79,91,84);
				glBegin(GL_QUADS);
				glVertex2f(x2,y1);
				glVertex2f(x3,y1);
				glVertex2f(x3,y2);
				glVertex2f(x2,y2);
				glEnd();

				Widgets::DropListItem *selected(component->getSelectedItem());
				if(selected)
				{
					glColor3ub(0,0,0);
					Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),selected->getText());
				}
			};

			Util::Size DefaultTheme::getDropListItemPreferedSize(Widgets::DropListItem *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
				return Util::Size(component->getRight()+component->getLeft()+text.width,20);
			};
			
			void DefaultTheme::paintDropListItem(Widgets::DropListItem *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getStatus()==Widgets::DropListItem::hover)
				{
					glColor3ub(175,200,28);
					glBegin(GL_QUADS);
					glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y));
					glVertex2f(static_cast<float>(origin.x+component->position.x+component->size.width),static_cast<float>(origin.y+component->position.y+component->size.height));
					glVertex2f(static_cast<float>(origin.x+component->position.x),static_cast<float>(origin.y+component->position.y+component->size.height));
					glEnd();
				}
				glColor3ub(0,0,0);
				Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->position.x+component->getLeft(),origin.y+component->position.y+component->getTop(),component->getText());
			};

			void DefaultTheme::paintDropDown(Util::Position &position,Util::Size &area)
			{
				glDisable(GL_TEXTURE_2D);
				glColor3ub(79,91,84);
				glBegin(GL_QUADS);
				glVertex2f(static_cast<float>(position.x),static_cast<float>(position.y));
				glVertex2f(static_cast<float>(position.x+area.width),static_cast<float>(position.y));
				glVertex2f(static_cast<float>(position.x+area.width),static_cast<float>(position.y+area.height));
				glVertex2f(static_cast<float>(position.x),static_cast<float>(position.y+area.height));
				glEnd();

				glColor3ub(46,55,53);
				glBegin(GL_LINE_STRIP);
				glVertex2f(static_cast<float>(position.x),static_cast<float>(position.y));
				glVertex2f(static_cast<float>(position.x+area.width),static_cast<float>(position.y));
				glVertex2f(static_cast<float>(position.x+area.width),static_cast<float>(position.y+area.height));
				glVertex2f(static_cast<float>(position.x),static_cast<float>(position.y+area.height));
				glVertex2f(static_cast<float>(position.x),static_cast<float>(position.y));
				glEnd();
			};

		DefaultTheme::~DefaultTheme(void)
		{
			uninstall();
		}
	}
}
