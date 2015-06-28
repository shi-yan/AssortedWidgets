#include "DropListManager.h"
#include "DropList.h"
#include "Graphics.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		DropListManager::DropListManager(void):currentDropped(0),isHover(false)
		{
		}

		void DropListManager::setDropped(Widgets::DropList *_currentDropped,int rx,int ry)
		{
			currentDropped=_currentDropped;
			position.x=currentX-rx;
			position.y=currentY-ry+22;
			size.width=0;
			size.height=0;
			unsigned int spacer(currentDropped->getSpacer());
			std::vector<Widgets::DropListItem*> &itemList=currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			int tempY=currentDropped->getTop();
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				Util::Size perfectSize=(*iter)->getPreferedSize();
                (*iter)->m_position.x=currentDropped->getLeft();
                (*iter)->m_position.y=tempY;
				size.width=std::max<unsigned int>(perfectSize.width,size.width);
				size.height+=spacer+perfectSize.height;
				tempY+=perfectSize.height+spacer;
			}

			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
                (*iter)->m_size.width=size.width;
			}

			size.width+=currentDropped->getLeft()+currentDropped->getRight();
			size.height+=currentDropped->getTop()+currentDropped->getBottom()-spacer;
		};

		void DropListManager::importMousePressed(Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;

			std::vector<Widgets::DropListItem*> &itemList=currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					currentDropped->setSelection((*iter));
					shrinkBack();
					return;
				}
			}

			shrinkBack();

		};

			void DropListManager::shrinkBack()
			{
				if(currentDropped)
				{
					currentDropped->shrinkBack();
					currentDropped=0;
				}
			};
		void  DropListManager::paint()
		{
			Theme::ThemeEngine::getSingleton().getTheme().paintDropDown(position,size);
            Util::Position p(position);
            Util::Graphics::getSingleton().pushPosition(p);
			std::vector<Widgets::DropListItem*> &itemList=currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				(*iter)->paint();
			}
			Util::Graphics::getSingleton().popPosition();
		};

		void DropListManager::importMouseEntered(Event::MouseEvent &e)
		{
			isHover=true;
			importMouseMotion(e);
		}

		void DropListManager::importMouseExited(Event::MouseEvent &e)
		{
			isHover=false;
			importMouseMotion(e);
		}

		void DropListManager::importMouseMotion(Event::MouseEvent &e)
		{
			int mx=e.getX()-position.x;
			int my=e.getY()-position.y;

			std::vector<Widgets::DropListItem*> &itemList=currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
					if((*iter)->isHover)
					{
						
					}
					else
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
						(*iter)->processMouseEntered(event);
					}
				}
				else
				{
					if((*iter)->isHover)
					{
						Event::MouseEvent event((*iter),Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						(*iter)->processMouseExited(event);
					}
				}
			}
		}

		DropListManager::~DropListManager(void)
		{
		}
	}
}
