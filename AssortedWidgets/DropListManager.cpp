#include "DropListManager.h"
#include "DropList.h"
#include "Graphics.h"

namespace AssortedWidgets
{
	namespace Manager
	{
        DropListManager::DropListManager(void)
            :m_currentDropped(0),
              m_isHover(false)
		{
		}

		void DropListManager::setDropped(Widgets::DropList *_currentDropped,int rx,int ry)
		{
            m_currentDropped=_currentDropped;
            m_position.x=m_currentX-rx;
            m_position.y=m_currentY-ry+22;
            m_size.m_width=0;
            m_size.m_height=0;
            unsigned int spacer(m_currentDropped->getSpacer());
            std::vector<Widgets::DropListItem*> &itemList=m_currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
            int tempY=m_currentDropped->getTop();
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				Util::Size perfectSize=(*iter)->getPreferedSize();
                (*iter)->m_position.x=m_currentDropped->getLeft();
                (*iter)->m_position.y=tempY;
                m_size.m_width=std::max<unsigned int>(perfectSize.m_width, m_size.m_width);
                m_size.m_height+=spacer+perfectSize.m_height;
                tempY+=perfectSize.m_height+spacer;
			}

			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
                (*iter)->m_size.m_width = m_size.m_width;
			}

            m_size.m_width += m_currentDropped->getLeft()+m_currentDropped->getRight();
            m_size.m_height += m_currentDropped->getTop()+m_currentDropped->getBottom() - spacer;
        }

		void DropListManager::importMousePressed(Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;

            std::vector<Widgets::DropListItem*> &itemList=m_currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
                    m_currentDropped->setSelection((*iter));
					shrinkBack();
					return;
				}
			}

			shrinkBack();

        }

        void DropListManager::shrinkBack()
        {
            if(m_currentDropped)
            {
                m_currentDropped->shrinkBack();
                m_currentDropped=0;
            }
        }

		void  DropListManager::paint()
		{
            Theme::ThemeEngine::getSingleton().getTheme().paintDropDown(m_position, m_size);
            Util::Position p(m_position);
            Util::Graphics::getSingleton().pushPosition(p);
            std::vector<Widgets::DropListItem*> &itemList = m_currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				(*iter)->paint();
			}
			Util::Graphics::getSingleton().popPosition();
        }

		void DropListManager::importMouseEntered(Event::MouseEvent &e)
		{
            m_isHover=true;
			importMouseMotion(e);
		}

		void DropListManager::importMouseExited(Event::MouseEvent &e)
		{
            m_isHover=false;
			importMouseMotion(e);
		}

		void DropListManager::importMouseMotion(Event::MouseEvent &e)
		{
            int mx=e.getX()-m_position.x;
            int my=e.getY()-m_position.y;

            std::vector<Widgets::DropListItem*> &itemList=m_currentDropped->getItemList();
			std::vector<Widgets::DropListItem*>::iterator iter;
			for(iter=itemList.begin();iter<itemList.end();++iter)
			{
				if((*iter)->isIn(mx,my))
				{
                    if((*iter)->m_isHover)
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
                    if((*iter)->m_isHover)
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
