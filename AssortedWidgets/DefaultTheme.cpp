#include "DefaultTheme.h"
#include "SDL2/SDL.h"
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
#include "DialogTitleBar.h"
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
#include "GraphicsBackend.h"

namespace AssortedWidgets
{
	namespace Theme
	{
		DefaultTheme::DefaultTheme(unsigned int _width,unsigned int _height)
		{
            m_screenWidth=_width;
            m_screenHeight=_height;
		}

		void DefaultTheme::test()
		{
            GraphicsBackend::getSingleton().drawTexturedQuad(0.0f,0.0f,256.0f,256.0f,
                                                             0.0f,0.0f,1.0f,1.0f,m_textureID);
        }

		void DefaultTheme::setup()
		{
            SDL_RWops *io = SDL_RWFromFile("assets/aw.png", "r+b");
            SDL_Surface *img=IMG_LoadPNG_RW(io);
		    SDL_LockSurface(img);   
            glGenTextures(1,&m_textureID);
            glBindTexture(GL_TEXTURE_2D,m_textureID);
			glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, img->w, img->h, 0, GL_RGBA, GL_UNSIGNED_BYTE, img->pixels);
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,GL_NEAREST);
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);

            SDL_UnlockSurface(img);
			SDL_FreeSurface(img);

            m_MenuLeft=new SubImage(1.0/256.0,1.0/256.0,7.0/256.0,21.0/256.0,m_textureID);
            m_MenuRight=new SubImage(53.0/256.0,1.0/256.0,59.0/256.0,21.0/256.0,m_textureID);

            m_MenuListUpLeft=new SubImage(3.0/256.0,34.0/256.0,27.0/256.0,43.0/256.0,m_textureID);
            m_MenuListUp=new SubImage(22.0/256.0,34.0/256.0,31.0/256.0,43.0/256.0,m_textureID);
            m_MenuListUpRight=new SubImage(33.0/256.0,34.0/256.0,57.0/256.0,43.0/256.0,m_textureID);
            m_MenuListLeft=new SubImage(3.0/256.0,38.0/256.0,27.0/256.0,43.0/256.0,m_textureID);
            m_MenuListRight=new SubImage(33.0/256.0,38.0/256.0,57.0/256.0,43.0/256.0,m_textureID);
            m_MenuListBottomLeft=new SubImage(3.0/256.0,44.0/256.0,27.0/256.0,60.0/256.0,m_textureID);
            m_MenuListBottom=new SubImage(22.0/256.0,44.0/256.0,31.0/256.0,60.0/256.0,m_textureID);
            m_MenuListBottomRight=new SubImage(33.0/256.0,44.0/256.0,57.0/256.0,60.0/256.0,m_textureID);
            m_MenuItemSubMenuArrow=new SubImage(62.0/256.0,1.0/256.0,67.0/256.0,10.0/256.0,m_textureID);
            m_ButtonNormalLeft=new SubImage(1.0/256.0,61.0/256.0,5.0/256.0,80.0/256.0,m_textureID);
            m_ButtonNormalRight=new SubImage(83.0/256.0,61.0/256.0,87.0/256.0,80.0/256.0,m_textureID);
            m_ButtonHoverLeft=new SubImage(1.0/256.0,81.0/256.0,5.0/256.0,100.0/256.0,m_textureID);
            m_ButtonHoverRight=new SubImage(83.0/256.0,81.0/256.0,87.0/256.0,100.0/256.0,m_textureID);
            m_RightHook=new SubImage(69.0/256.0,1.0/256.0,77.0/256.0,10.0/256.0,m_textureID);
            m_RadioDot=new SubImage(78.0/256.0,1.0/256.0,87.0/256.0,10.0/256.0,m_textureID);
            m_DialogUpLeftActive=new SubImage(3.0/256.0,27.0/256.0,27.0/256.0,43.0/256.0,m_textureID);
            m_DialogUpActive=new SubImage(22.0/256.0,27.0/256.0,31.0/256.0,43.0/256.0,m_textureID);
            m_DialogUpRightActive=new SubImage(33.0/256.0,27.0/256.0,57.0/256.0,43.0/256.0,m_textureID);
            m_DialogLeft=new SubImage(3.0/256.0,38.0/256.0,27.0/256.0,43.0/256.0,m_textureID);
            m_DialogRight=new SubImage(33.0/256.0,38.0/256.0,57.0/256.0,43.0/256.0,m_textureID);
            m_DialogBottomLeft=new SubImage(3.0/256.0,44.0/256.0,27.0/256.0,60.0/256.0,m_textureID);
            m_DialogBottom=new SubImage(22.0/256.0,44.0/256.0,31.0/256.0,60.0/256.0,m_textureID);
            m_DialogBottomRight=new SubImage(33.0/256.0,44.0/256.0,57.0/256.0,60.0/256.0,m_textureID);
            m_TextFieldLeft=new SubImage(1.0/256.0,101.0/256.0,5.0/256.0,121.0/256.0,m_textureID);
            m_TextFieldRight=new SubImage(47.0/256.0,101.0/256.0,51.0/256.0,121.0/256.0,m_textureID);
            m_Logo=new SubImage(0.0/256.0,169.0/256.0,253.0/256.0,255.0/256.0,m_textureID);

            m_ScrollBarVerticalTopNormal=new SubImage(1.0/256.0,122.0/256.0,16.0/256.0,137.0/256.0,m_textureID);
            m_ScrollBarVerticalBottomNormal=new SubImage(33.0/256.0,122.0/256.0,48.0/256.0,137.0/256.0,m_textureID);
            m_ScrollBarHorizontalLeftNormal=new SubImage(17.0/256.0,122.0/256.0,32.0/256.0,137.0/256.0,m_textureID);
            m_ScrollBarHorizontalRightNormal=new SubImage(49.0/256.0,122.0/256.0,64.0/256.0,137.0/256.0,m_textureID);

            m_ScrollBarVerticalTopHover=new SubImage(81.0/256.0,101.0/256.0,96.0/256.0,116.0/256.0,m_textureID);
            m_ScrollBarVerticalBottomHover=new SubImage(113.0/256.0,101.0/256.0,128.0/256.0,116.0/256.0,m_textureID);
            m_ScrollBarHorizontalLeftHover=new SubImage(97.0/256.0,101.0/256.0,112.0/256.0,116.0/256.0,m_textureID);
            m_ScrollBarHorizontalRightHover=new SubImage(129.0/256.0,101.0/256.0,143.0/256.0,116.0/256.0,m_textureID);

            m_ScrollBarHorizontalBackground=new SubImage(2.0/256.0,138.0/256.0,12.0/256.0,153.0/256.0,m_textureID);
            m_ScrollBarVerticalBackground=new SubImage(65.0/256.0,102.0/256.0,80.0/256.0,112.0/256.0,m_textureID);

            m_CheckButtonOn=new SubImage(81.0/256.0,129.0/256.0,92.0/256.0,140.0/256.0,m_textureID);
            m_CheckButtonOff=new SubImage(94.0/256.0,129.0/256.0,105.0/256.0,140.0/256.0,m_textureID);
            m_RadioButtonOn=new SubImage(81.0/256.0,117.0/256.0,92.0/256.0,128.0/256.0,m_textureID);
            m_RadioButtonOff=new SubImage(94.0/256.0,117.0/256.0,105.0/256.0,128.0/256.0,m_textureID);

            m_ProgressBarLeft=new SubImage(1.0/256.0,101.0/256.0,5.0/256.0,121.0/256.0,m_textureID);
            m_ProgressBarRight=new SubImage(47.0/256.0,101.0/256.0,51.0/256.0,121.0/256.0,m_textureID);
            m_ProgressBarTop=new SubImage(106.0/256.0,117.0/256.0,126.0/256.0,121.0/256.0,m_textureID);
            m_ProgressBarBottom=new SubImage(106.0/256.0,145.0/256.0,126.0/256.0,149.0/256.0,m_textureID);

            m_DialogUpLeftDeactive=new SubImage(89.0/256.0,61.0/256.0,113.0/256.0,77.0/256.0,m_textureID);
            m_DialogUpDeactive=new SubImage(111.0/256.0,61.0/256.0,116.0/256.0,77.0/256.0,m_textureID);
            m_DialogUpRightDeactive=new SubImage(119.0/256.0,61.0/256.0,143.0/256.0,77.0/256.0,m_textureID);
        }

		Util::Size DefaultTheme::getMenuPreferedSize(Widgets::Menu *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
            return Util::Size(12+text.m_width,19);
        }

		void DefaultTheme::paintMenu(Widgets::Menu *component)
		{
			if(component->isExpand())
			{
                float x1=static_cast<float>(component->m_position.x);
                float y1=static_cast<float>(component->m_position.y);
                float x2=static_cast<float>(component->m_position.x+component->m_size.m_width);
                float y2=static_cast<float>(component->m_position.y+component->m_size.m_height);

                GraphicsBackend::getSingleton().drawSolidQuad(x1+6,y1,x2-6,y2,44,55,55);

                m_MenuLeft->paint(x1,y1,x1+6,y2);
                m_MenuRight->paint(x2-6,y1,x2,y2);
                Font::FontEngine::getSingleton().getFont().setColor(150,155,161);
                Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
			}
			else
			{
				switch(component->getStatus())
				{
					case Widgets::Menu::hover :
					{
                        float x1=static_cast<float>(component->m_position.x);
                        float y1=static_cast<float>(component->m_position.y);
                        float x2=static_cast<float>(component->m_position.x+component->m_size.m_width);
                        float y2=static_cast<float>(component->m_position.y+component->m_size.m_height);

                        GraphicsBackend::getSingleton().drawSolidQuad(x1+6,y1,x2-6,y2,44,55,55);
                        m_MenuLeft->paint(x1,y1,x1+6,y2);
                        m_MenuRight->paint(x2-6,y1,x2,y2);
                        Font::FontEngine::getSingleton().getFont().setColor(150,155,161);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
					case Widgets::Menu::normal:
					{
                        float x1=static_cast<float>(component->m_position.x);
                        float y1=static_cast<float>(component->m_position.y);
                        Font::FontEngine::getSingleton().getFont().setColor(150,155,161);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
					case Widgets::Menu::pressed:
					{
                        float x1=static_cast<float>(component->m_position.x);
                        float y1=static_cast<float>(component->m_position.y);
                        Font::FontEngine::getSingleton().getFont().setColor(250,250,250);
						Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x1+6),static_cast<int>(y1+6),component->getText());
						break;
					}
				}
			}
        }

		Util::Size DefaultTheme::getMenuBarPreferedSize(Widgets::MenuBar *component)
		{
            return Util::Size(component->m_size.m_width,30);
        }

		void DefaultTheme::paintMenuBar(Widgets::MenuBar *component)
		{
			float x1=0.0f;
			float y1=0.0f;
            float x2=static_cast<float>(component->m_size.m_width);
			float y2=30.0f;

            GraphicsBackend::getSingleton().drawSolidQuad(x1, y1, x1 + 40.0f, y2, 42, 55, 55);
            GraphicsBackend::getSingleton().drawSolidQuad(x1+40.0f,y1,x2,y2,55,65,67);
        }

		Util::Size DefaultTheme::getMenuListPreferedSize(Widgets::MenuList *component)
		{
			unsigned int miniSize(component->getMinimizeSize());
			unsigned int width(0);
			unsigned int height(0);

			std::vector<Widgets::MenuItem *>::iterator iter;
			for(iter=component->getItemList().begin();iter<component->getItemList().end();++iter)
			{
				Util::Size itemSize=(*iter)->getPreferedSize();
                width=std::max<unsigned int>(width,itemSize.m_width);
                height+=itemSize.m_height;
			}

			return Util::Size(width+component->getLeft()+component->getRight(),height+component->getTop()+component->getBottom());
        }

		void DefaultTheme::paintMenuList(Widgets::MenuList *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
            float x1=static_cast<float>(origin.x+component->m_position.x);
            float y1=static_cast<float>(origin.y+component->m_position.y);
            float x2=static_cast<float>(x1+component->m_size.m_width);
            float y2=static_cast<float>(y1+component->m_size.m_height);
            m_MenuListUpLeft->paint(x1,y1,x1+24.0f,y1+9.0f);
            m_MenuListUpRight->paint(x2-24.0f,y1,x2,y1+9.0f);
            m_MenuListUp->paint(x1+24.0f,y1,x2-24.0f,y1+9.0f);
            m_MenuListLeft->paint(x1,y1+9.0f,x1+24.0f,y2-16.0f);
            m_MenuListRight->paint(x2-24.0f,y1+9.0f,x2,y2-16.0f);
            m_MenuListBottomLeft->paint(x1,y2-16.0f,x1+24.0f,y2);
            m_MenuListBottomRight->paint(x2-24.0f,y2-16.0f,x2,y2);
            m_MenuListBottom->paint(x1+24.0f,y2-16.0f,x2-24.0f,y2);

            GraphicsBackend::getSingleton().drawSolidQuad(x1+24.0f,y1+9.0f,x2-24.0f,y2-16.0f,46,55,53);
        }

		Util::Size DefaultTheme::getMenuItemButtonPreferedSize(Widgets::MenuItemButton *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
            return Util::Size(24+text.m_width,20);
        }

		Util::Size DefaultTheme::getMenuItemSeparatorPreferedSize(Widgets::MenuItemSeparator *component)
		{
            return Util::Size(component->m_size);
        }

		void DefaultTheme::paintMenuItemSeparator(Widgets::MenuItemSeparator *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();

            GraphicsBackend::getSingleton().drawLine(static_cast<float>(10+origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y+1),
                                                     static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-10),static_cast<float>(origin.y+component->m_position.y+1),
                                                     79,91,84);
        }

		void DefaultTheme::paintMenuItemButton(Widgets::MenuItemButton *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			switch(component->getStatus())
			{
				case Widgets::MenuItemButton::normal:
				{
                    Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
					break;
				};
				case Widgets::MenuItemButton::pressed:
				{
                    Font::FontEngine::getSingleton().getFont().setColor(200,200,200);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
					break;
				};
				case Widgets::MenuItemButton::hover:
				{
                    GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(component->m_position.x+origin.x), static_cast<float>(origin.y+component->m_position.y),
                                                                  static_cast<float>(component->m_position.x+origin.x+component->m_size.m_width),
                                                                  static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                  176,200,28);

                    Font::FontEngine::getSingleton().getFont().setColor(88,101,9);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
					break;
				};
			}
		}

		Util::Size DefaultTheme::getMenuItemSubMenuPreferedSize(Widgets::MenuItemSubMenu *component)
		{
			Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
            return Util::Size(24+10+text.m_width,20);
		}
			
		void DefaultTheme::paintMenuItemSubMenu(Widgets::MenuItemSubMenu *component)
		{
			Util::Position origin=Util::Graphics::getSingleton().getOrigin();
			switch(component->getStatus())
			{
				case Widgets::MenuItemSubMenu::normal:
				{
                    Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());

					if(component->isExpand())
					{
                        m_MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-17),static_cast<float>(component->m_position.y+origin.y+5),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-12),static_cast<float>(component->m_position.y+origin.y+14));
					}
					else
					{
                        m_MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-22),static_cast<float>(component->m_position.y+origin.y+5),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-17),static_cast<float>(component->m_position.y+origin.y+14));
					}
					break;
				};
				case Widgets::MenuItemSubMenu::pressed:
				{
                    Font::FontEngine::getSingleton().getFont().setColor(200,200,200);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
					break;
				};
				case Widgets::MenuItemSubMenu::hover:
				{
                    GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(component->m_position.x+origin.x), static_cast<float>(origin.y+component->m_position.y),
                                                                  static_cast<float>(component->m_position.x+origin.x+component->m_size.m_width),
                                                                  static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                  176,200,28);

                    Font::FontEngine::getSingleton().getFont().setColor(88,101,9);
                    Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
					if(component->isExpand())
					{
                        m_MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-17),static_cast<float>(component->m_position.y+origin.y+5),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-12),static_cast<float>(component->m_position.y+origin.y+14));
					}
					else
					{
                        m_MenuItemSubMenuArrow->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-22),static_cast<float>(component->m_position.y+origin.y+5),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-17),static_cast<float>(component->m_position.y+origin.y+14));
					}
					break;
				};
			}
		}

            Util::Size DefaultTheme::getLabelPreferedSize(Widgets::Label *component) const
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(component->getRight()+component->getLeft()+text.m_width,20);
            }

			void DefaultTheme::paintLabel(Widgets::Label *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->isDrawBackground())
				{
                    GraphicsBackend::getSingleton().drawSolidQuad(static_cast<GLfloat>(origin.x+component->m_position.x),
                                                                  static_cast<GLfloat>(origin.y+component->m_position.y),
                                                                  static_cast<GLfloat>(origin.x+component->m_position.x+component->m_size.m_width),
                                                                  static_cast<GLfloat>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                  0,0,0);
				}
                Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),component->getText());
            }

			Util::Size DefaultTheme::getButtonPreferedSize(Widgets::Button *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(component->getRight()+component->getLeft()+text.m_width,19);
            }
			
			void DefaultTheme::paintButton(Widgets::Button *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::Button::normal:
					{
                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);


                        Font::FontEngine::getSingleton().getFont().setColor(137,155,145);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),component->getText());
						break;
					};

					case Widgets::Button::hover:
					{
                        m_ButtonHoverLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonHoverRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      175,200,28);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),component->getText());
						break;
					};

					case Widgets::Button::pressed:
					{
                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),component->getText());
						break;
					};
				}
            }

			Util::Size DefaultTheme::getMenuItemToggleButtonPreferedSize(Widgets::MenuItemToggleButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(10+24+text.m_width,20);
            }

			void DefaultTheme::paintMenuItemToggleButton(Widgets::MenuItemToggleButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::MenuItemToggleButton::normal:
					{
                        Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RightHook->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
					case Widgets::MenuItemToggleButton::pressed:
					{
                        Font::FontEngine::getSingleton().getFont().setColor(200,200,200);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RightHook->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
					case Widgets::MenuItemToggleButton::hover:
					{
                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(component->m_position.x+origin.x),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(component->m_position.x+origin.x+component->m_size.m_width),
                                                                      static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                      176,200,28);

                        Font::FontEngine::getSingleton().getFont().setColor(88,101,9);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RightHook->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
				}
            }

			Util::Size DefaultTheme::getMenuItemRadioButtonPreferedSize(Widgets::MenuItemRadioButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(10+24+text.m_width,20);
            }

			void DefaultTheme::paintMenuItemRadioButton(Widgets::MenuItemRadioButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				switch(component->getStatus())
				{
					case Widgets::MenuItemRadioButton::normal:
					{
                        Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RadioDot->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
					case Widgets::MenuItemRadioButton::pressed:
					{
                        Font::FontEngine::getSingleton().getFont().setColor(200,200,200);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RadioDot->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
					case Widgets::MenuItemRadioButton::hover:
					{

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(component->m_position.x+origin.x),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(component->m_position.x+origin.x+component->m_size.m_width),
                                                                      static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                      176,200,28);

                        Font::FontEngine::getSingleton().getFont().setColor(88,101,9);
                        Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(10+component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
						if(component->getToggle())
						{
                            m_RadioDot->paint(static_cast<float>(component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y),static_cast<float>(8+component->m_position.x+component->getLeft()+origin.x),static_cast<float>(component->getTop()+origin.y+component->m_position.y+9));
						}
						break;
					};
				}
            }
			
			Util::Size DefaultTheme::getMenuItemRadioGroupPreferedSize(Widgets::MenuItemRadioGroup *component)
			{
				unsigned int miniSize(component->getMinimizeSize());
				unsigned int width(0);
				unsigned int height(0);

				std::vector<Widgets::MenuItemRadioButton *>::iterator iter;
				for(iter=component->getItemList().begin();iter<component->getItemList().end();++iter)
				{
					Util::Size itemSize=(*iter)->getPreferedSize();
                    width=std::max<unsigned int>(width,itemSize.m_width);
                    height+=itemSize.m_height;
				}
				return Util::Size(width+component->getLeft()+component->getRight(),height+component->getTop()+component->getBottom());
            }
			
			void DefaultTheme::paintMenuItemRadioGroup(Widgets::MenuItemRadioGroup *component)
			{
            }

			Util::Size DefaultTheme::getDialogPreferedSize(Widgets::Dialog *component)
			{
                return component->m_size;
            }

			void DefaultTheme::paintDialog(Widgets::Dialog *component)
			{
                float x1=static_cast<float>(component->m_position.x+24);
                float x2=static_cast<float>(component->m_position.x+component->m_size.m_width-24);
                float y1=static_cast<float>(component->m_position.y+component->m_size.m_height-16);
                float y2=static_cast<float>(component->m_position.y+component->m_size.m_height);

				if(component->isActive())
				{
                    m_DialogUpLeftActive->paint(static_cast<float>(component->m_position.x),static_cast<float>(component->m_position.y),static_cast<float>(x1),static_cast<float>(component->m_position.y+16));
                    m_DialogUpActive->paint(static_cast<float>(x1),static_cast<float>(component->m_position.y),static_cast<float>(x2),static_cast<float>(component->m_position.y+16));
                    m_DialogUpRightActive->paint(static_cast<float>(x2),static_cast<float>(component->m_position.y),static_cast<float>(component->m_position.x+component->m_size.m_width),static_cast<float>(component->m_position.y+16));
				}
				else
				{
                    m_DialogUpLeftDeactive->paint(static_cast<float>(component->m_position.x),static_cast<float>(component->m_position.y),static_cast<float>(x1),static_cast<float>(component->m_position.y+16));
                    m_DialogUpDeactive->paint(static_cast<float>(x1),static_cast<float>(component->m_position.y),static_cast<float>(x2),static_cast<float>(component->m_position.y+16));
                    m_DialogUpRightDeactive->paint(static_cast<float>(x2),static_cast<float>(component->m_position.y),static_cast<float>(component->m_position.x+component->m_size.m_width),static_cast<float>(component->m_position.y+16));
				}

                m_DialogLeft->paint(static_cast<float>(component->m_position.x),static_cast<float>(component->m_position.y+16),static_cast<float>(x1),static_cast<float>(y1));
                m_DialogRight->paint(static_cast<float>(x2),static_cast<float>(component->m_position.y+16),static_cast<float>(component->m_position.x+component->m_size.m_width),static_cast<float>(y1));
                m_DialogBottomLeft->paint(static_cast<float>(component->m_position.x),static_cast<float>(y1),static_cast<float>(x1),static_cast<float>(y2));
                m_DialogBottom->paint(static_cast<float>(x1),static_cast<float>(y1),static_cast<float>(x2),static_cast<float>(y2));
                m_DialogBottomRight->paint(static_cast<float>(x2),static_cast<float>(y1),static_cast<float>(component->m_position.x+component->m_size.m_width),static_cast<float>(y2));

                GraphicsBackend::getSingleton().drawSolidQuad(x1,component->m_position.y+16,x2,y1,
                                                              46,55,53);
            }

            Util::Size DefaultTheme::getDialogTitleBarPreferedSize(Widgets::DialogTitleBar *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(20+text.m_width,20);
            }
			
            void DefaultTheme::paintDialogTitleBar(Widgets::DialogTitleBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

                GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x),
                                                              static_cast<float>(origin.y+component->m_position.y),
                                                              static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),
                                                              static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                              31,31,31);

                Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(component->m_position.x+component->getLeft()+origin.x),static_cast<int>(component->getTop()+origin.y+component->m_position.y),component->getText());
            }

			Util::Size DefaultTheme::getTextFieldPreferedSize(Widgets::TextField *component)
			{
				return Util::Size(component->getLength()+12,20);
            }

			void DefaultTheme::paintTextField(Widgets::TextField *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
                float x1=static_cast<float>(origin.x+component->m_position.x);
                float x2=static_cast<float>(origin.x+component->m_position.x+4);
                float x3=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4);
                float x4=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                float y1=static_cast<float>(origin.y+component->m_position.y);
                float y2=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                m_TextFieldLeft->paint(x1,y1,x2,y2);
                m_TextFieldRight->paint(x3,y1,x4,y2);

                GraphicsBackend::getSingleton().drawSolidQuad(x2,y1,x3,y2,79,91,84);

				if(component->isActive())
				{
                    GraphicsBackend::getSingleton().drawSolidQuad(x3+2,y1+4,x3+3,y2-4,0,0,0);
				}
				Util::Size textSize=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                glEnable(GL_SCISSOR_TEST);
                glScissor(static_cast<GLint>(x1),static_cast<GLint>(m_screenHeight-y2),static_cast<GLint>(component->m_size.m_width),static_cast<GLint>(component->m_size.m_height));
                Font::FontEngine::getSingleton().getFont().drawString(static_cast<int>(x3-4-textSize.m_width),static_cast<int>(component->getTop()+y1),component->getText());
                glDisable(GL_SCISSOR_TEST);
            }

			Util::Size DefaultTheme::getLogoPreferedSize(Widgets::Logo *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintLogo(Widgets::Logo *component)
			{
                Font::FontEngine::getSingleton().getFont().setColor(255,255,255);
                m_Logo->paint(static_cast<float>(component->m_position.x),static_cast<float>(component->m_position.y),static_cast<float>(component->m_position.x+component->m_size.m_width),static_cast<float>(component->m_position.y+component->m_size.m_height));
            }

			Util::Size DefaultTheme::getScrollBarButtonPreferedSize(Widgets::ScrollBarButton *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintScrollBarButton(Widgets::ScrollBarButton *component)
			{
				SubImage *button=0;
				switch(component->getType())
				{
					case Widgets::ScrollBarButton::HorizontalLeft:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
                            button = m_ScrollBarHorizontalLeftNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
                            button = m_ScrollBarHorizontalLeftHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
                            button = m_ScrollBarHorizontalLeftNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::HorizontalRight:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
                            button = m_ScrollBarHorizontalRightNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
                            button = m_ScrollBarHorizontalRightHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
                            button = m_ScrollBarHorizontalRightNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::VerticalTop:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
                            button = m_ScrollBarVerticalTopNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
                            button = m_ScrollBarVerticalTopHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
                            button = m_ScrollBarVerticalTopNormal;
						}
						break;
					}
					case Widgets::ScrollBarButton::VerticalBottom:
					{
						if(component->getStatus()==Widgets::ScrollBarButton::normal)
						{
                            button = m_ScrollBarVerticalBottomNormal;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::hover)
						{
                            button = m_ScrollBarVerticalBottomHover;
						}
						else if(component->getStatus()==Widgets::ScrollBarButton::pressed)
						{
                            button = m_ScrollBarVerticalBottomNormal;
						}
						break;
					}
				}
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
                button->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height));
            }

			Util::Size DefaultTheme::getScrollBarSliderPreferedSize(Widgets::ScrollBarSlider *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintScrollBarSlider(Widgets::ScrollBarSlider *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

                GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x),
                                                              static_cast<float>(origin.y+component->m_position.y),
                                                              static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),
                                                              static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                              46,55,53);
            }

            Util::Size DefaultTheme::getScrollBarPreferedSize(Widgets::ScrollBar *)
			{
				return Util::Size();
            }
						
			void DefaultTheme::paintScrollBar(Widgets::ScrollBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

				if(component->getType()==Widgets::ScrollBar::Horizontal)
				{
                    m_ScrollBarHorizontalBackground->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height));
				}
				else if(component->getType()==Widgets::ScrollBar::Vertical)
				{
                    m_ScrollBarVerticalBackground->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height));
				}		
            }

            Util::Size DefaultTheme::getScrollPanelPreferedSize(Widgets::ScrollPanel *)
			{
				return Util::Size();
            }

			void DefaultTheme::paintScrollPanel(Widgets::ScrollPanel *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

                GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x),
                                                              static_cast<float>(origin.y+component->m_position.y),
                                                              static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),
                                                              static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                              79,91,84);

				if(component->isHorizontalBarShow()||component->isVerticalBarShow())
				{

                    GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-15),
                                                                  static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height-15),
                                                                  static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-1),
                                                                  static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height-1),
                                                                  46,55,53);
				}
            }

			void DefaultTheme::scissorBegin(Util::Position &position,Util::Size &area)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

				glEnable(GL_SCISSOR_TEST);
                glScissor(origin.x+position.x,m_screenHeight-origin.y-area.m_height-position.y,area.m_width,area.m_height);

            }

			void DefaultTheme::scissorEnd()
			{
				glDisable(GL_SCISSOR_TEST);
            }

			Util::Size DefaultTheme::getCheckButtonPreferedSize(Widgets::CheckButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(component->getRight()+component->getLeft()+text.m_width+15,19);
            }

			void DefaultTheme::paintCheckButton(Widgets::CheckButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				SubImage *checkStatus(0);
				if(component->isCheck())
				{
                    checkStatus=m_CheckButtonOn;
				}
				else
				{
                    checkStatus=m_CheckButtonOff;
				}
				switch(component->getStatus())
				{
					case Widgets::CheckButton::normal:
					{

                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);

                        Font::FontEngine::getSingleton().getFont().setColor(137,155,145);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::hover:
					{
                        m_ButtonHoverLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonHoverRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      175,200,28);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::pressed:
					{
                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};
				}
            }

			Util::Size DefaultTheme::getRadioButtonPreferedSize(Widgets::RadioButton *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(component->getRight()+component->getLeft()+text.m_width+15,19);
            }

			void DefaultTheme::paintRadioButton(Widgets::RadioButton *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				SubImage *checkStatus(0);
				if(component->isCheck())
				{
                    checkStatus=m_RadioButtonOn;
				}
				else
				{
                    checkStatus=m_RadioButtonOff;
				}
				switch(component->getStatus())
				{
					case Widgets::CheckButton::normal:
					{
                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);

                        Font::FontEngine::getSingleton().getFont().setColor(137,155,145);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::hover:
					{
                        m_ButtonHoverLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonHoverRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      175,200,28);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};

					case Widgets::CheckButton::pressed:
					{
                        m_ButtonNormalLeft->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+4),static_cast<float>(origin.y+component->m_position.y+19));
                        m_ButtonNormalRight->paint(static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+19));

                        GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x+4),
                                                                      static_cast<float>(origin.y+component->m_position.y),
                                                                      static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4),
                                                                      static_cast<float>(origin.y+component->m_position.y+19),
                                                                      55,67,65);

                        Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                        Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft()+15,origin.y+component->m_position.y+component->getTop(),component->getText());

                        checkStatus->paint(static_cast<float>(origin.x+component->m_position.x+component->getLeft()),static_cast<float>(origin.y+component->m_position.y+component->getTop()),static_cast<float>(origin.x+component->m_position.x+component->getLeft()+11),static_cast<float>(origin.y+component->m_position.y+component->getTop()+11));
						break;
					};
				}
            }

			Util::Size DefaultTheme::getProgressBarPreferedSize(Widgets::ProgressBar *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintProgressBar(Widgets::ProgressBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getType()==Widgets::ProgressBar::Horizontal)
				{
					Util::Position origin=Util::Graphics::getSingleton().getOrigin();
                    float x1=static_cast<float>(origin.x+component->m_position.x);
                    float x2=static_cast<float>(origin.x+component->m_position.x+4);
                    float x3=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4);
                    float x4=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                    float y1=static_cast<float>(origin.y+component->m_position.y);
                    float y2=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                    m_ProgressBarLeft->paint(x1,y1,x2,y2);
                    m_ProgressBarRight->paint(x3,y1,x4,y2);

                    GraphicsBackend::getSingleton().drawSolidQuad(x2,y1,x3,y2,79,91,84);
                    GraphicsBackend::getSingleton().drawSolidQuad(x1+2,y1+2,x1+2+component->getPOfSlider(),y2-2,46,55,53);
				}
				else if(component->getType()==Widgets::ProgressBar::Vertical)
				{

                    float x1=static_cast<float>(origin.x+component->m_position.x);
                    float x2=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                    float y1=static_cast<float>(origin.y+component->m_position.y);
                    float y2=static_cast<float>(origin.y+component->m_position.y+4);
                    float y3=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height-4);
                    float y4=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                    m_ProgressBarTop->paint(x1,y1,x2,y2);
                    m_ProgressBarBottom->paint(x1,y3,x2,y4);

                    GraphicsBackend::getSingleton().drawSolidQuad(x1,y2,x2,y3,79,91,84);

                    GraphicsBackend::getSingleton().drawSolidQuad(x1+2,y4-2-component->getPOfSlider(),x2-2,y4-2,46,55,53);
				}
            }

			Util::Size DefaultTheme::getSlideBarSliderPreferedSize(Widgets::SlideBarSlider *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintSlideBarSlider(Widgets::SlideBarSlider *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();

                GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x),
                                                              static_cast<float>(origin.y+component->m_position.y),
                                                              static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),
                                                              static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                              46,55,53);
            }

			Util::Size DefaultTheme::getSlideBarPreferedSize(Widgets::SlideBar *component)
			{
				return Util::Size();
            }

			void DefaultTheme::paintSlideBar(Widgets::SlideBar *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getType()==Widgets::SlideBar::Horizontal)
				{
					Util::Position origin=Util::Graphics::getSingleton().getOrigin();
                    float x1=static_cast<float>(origin.x+component->m_position.x);
                    float x2=static_cast<float>(origin.x+component->m_position.x+4);
                    float x3=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4);
                    float x4=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                    float y1=static_cast<float>(origin.y+component->m_position.y);
                    float y2=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                    m_ProgressBarLeft->paint(x1,y1,x2,y2);
                    m_ProgressBarRight->paint(x3,y1,x4,y2);

                    GraphicsBackend::getSingleton().drawSolidQuad(x2,y1,x3,y2,79,91,84);

				}
				else if(component->getType()==Widgets::SlideBar::Vertical)
				{
                    float x1=static_cast<float>(origin.x+component->m_position.x);
                    float x2=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                    float y1=static_cast<float>(origin.y+component->m_position.y);
                    float y2=static_cast<float>(origin.y+component->m_position.y+4);
                    float y3=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height-4);
                    float y4=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                    m_ProgressBarTop->paint(x1,y1,x2,y2);
                    m_ProgressBarBottom->paint(x1,y3,x2,y4);

                    GraphicsBackend::getSingleton().drawSolidQuad(x1,y2,x2,y3,79,91,84);
				}
            }

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
                        button = m_ScrollBarVerticalBottomNormal;
						break;
					}
					case Widgets::DropListButton::hover:
					{
                        button = m_ScrollBarVerticalBottomHover;
						break;
					}
					case Widgets::DropListButton::pressed:
					{
                        button = m_ScrollBarVerticalBottomNormal;
						break;
					}
				}

                Util::Position origin=Util::Graphics::getSingleton().getOrigin();

                button->paint(static_cast<float>(origin.x+component->m_position.x),static_cast<float>(origin.y+component->m_position.y),static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height));
            }

            Util::Size DefaultTheme::getDropListPreferedSize(Widgets::DropList *)
			{
				return Util::Size();
            }

			void DefaultTheme::paintDropList(Widgets::DropList *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
                float x1=static_cast<float>(origin.x+component->m_position.x);
                float x2=static_cast<float>(origin.x+component->m_position.x+4);
                float x3=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width-4);
                float x4=static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width);
                float y1=static_cast<float>(origin.y+component->m_position.y);
                float y2=static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height);

                m_ProgressBarLeft->paint(x1,y1,x2,y2);
                m_ProgressBarRight->paint(x3,y1,x4,y2);

                GraphicsBackend::getSingleton().drawSolidQuad(x2,y1,x3,y2,79,91,84);

				Widgets::DropListItem *selected(component->getSelectedItem());
				if(selected)
				{
                    Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                    Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),selected->getText());
				}
            }

			Util::Size DefaultTheme::getDropListItemPreferedSize(Widgets::DropListItem *component)
			{
				Util::Size text=Font::FontEngine::getSingleton().getFont().getStringBoundingBox(component->getText());
                return Util::Size(component->getRight()+component->getLeft()+text.m_width,20);
            }
			
			void DefaultTheme::paintDropListItem(Widgets::DropListItem *component)
			{
				Util::Position origin=Util::Graphics::getSingleton().getOrigin();
				if(component->getStatus()==Widgets::DropListItem::hover)
				{

                    GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(origin.x+component->m_position.x),
                                                                  static_cast<float>(origin.y+component->m_position.y),
                                                                  static_cast<float>(origin.x+component->m_position.x+component->m_size.m_width),
                                                                  static_cast<float>(origin.y+component->m_position.y+component->m_size.m_height),
                                                                  175,200,28);
				}
                Font::FontEngine::getSingleton().getFont().setColor(0,0,0);
                Font::FontEngine::getSingleton().getFont().drawString(origin.x+component->m_position.x+component->getLeft(),origin.y+component->m_position.y+component->getTop(),component->getText());
            }

			void DefaultTheme::paintDropDown(Util::Position &position,Util::Size &area)
			{

                GraphicsBackend::getSingleton().drawSolidQuad(static_cast<float>(position.x),
                                                              static_cast<float>(position.y),
                                                              static_cast<float>(position.x+area.m_width),
                                                              static_cast<float>(position.y+area.m_height),
                                                              79,91,84);


                std::vector<float> points = {static_cast<float>(position.x),static_cast<float>(position.y),
                                      static_cast<float>(position.x+area.m_width),static_cast<float>(position.y),
                                      static_cast<float>(position.x+area.m_width),static_cast<float>(position.y+area.m_height),
                                      static_cast<float>(position.x),static_cast<float>(position.y+area.m_height),
                                      static_cast<float>(position.x),static_cast<float>(position.y)};


                GraphicsBackend::getSingleton().drawLineStrip(points, 46,55,53);


            }

		DefaultTheme::~DefaultTheme(void)
		{
			uninstall();
		}
	}
}
