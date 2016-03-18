#include "SDL2/SDL.h"
#include "SDL2/SDL_opengl.h"
#include "UI.h"
#include <string>
#include <QApplication>
#include <QDebug>
#include "SDL2/SDL_image.h"
#include "fontstash.h"

//The window we'll be rendering to
SDL_Window* window = NULL;

//The surface contained by the window
SDL_Surface* screenSurface = NULL;
void init(int width,int height)
{
	bool fullscreen =true;
	int bpp = 32;
	int flags = 0;

	if(SDL_Init(SDL_INIT_VIDEO) < 0)
	{
		printf("Video initialization failed: %s\n", SDL_GetError());
	}

    //Use OpenGL 2.1
    SDL_GL_SetAttribute( SDL_GL_CONTEXT_MAJOR_VERSION, 2 );
    SDL_GL_SetAttribute( SDL_GL_CONTEXT_MINOR_VERSION, 1 );

    //if(!fullscreen)
    //	flags = SDL_OPENGL;
    //else
    //	flags = SDL_OPENGL | SDL_FULLSCREEN;
			
    /*if(SDL_SetVideoMode(width, height, bpp, flags) == 0)
	{
		printf("Video mode set failed: %s\n", SDL_GetError());
	}

	SDL_WM_SetCaption("Assorted Widgets",0);
    SDL_EnableUNICODE(1); */


    window = SDL_CreateWindow( "Assorted Widgets", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, width, height, SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN );
            if( window == NULL )
            {
                qDebug() << "Window could not be created! SDL_Error: "<< SDL_GetError() ;
            }

            else{
                int imgFlags = IMG_INIT_PNG;
                        if( !( IMG_Init( imgFlags ) & imgFlags ) )
                        {
                            qDebug() <<  "SDL_image could not initialize! SDL_image Error: "<< IMG_GetError() ;
                            //success = false;
                        }

                SDL_GLContext gContext = SDL_GL_CreateContext( window );
                            if( gContext == NULL )
                            {
                                qDebug() <<  "OpenGL context could not be created! SDL Error: "<< SDL_GetError() ;
                                //success = false;
                            }
                            else
                            {
                                //Use Vsync
                                if( SDL_GL_SetSwapInterval( 1 ) < 0 )
                                {
                                    printf( "Warning: Unable to set VSync! SDL Error: %s\n", SDL_GetError() );
                                }

                                //Initialize OpenGL
                               /* if( !initGL() )
                                {
                                    printf( "Unable to initialize OpenGL!\n" );
                                    success = false;
                                }*/
                            }

            }

	glShadeModel(GL_SMOOTH);
	glClearColor(118.0f/255.0f,130.0f/255.0f,123.0f/255.0f, 1.0f);
	glClearDepth(1.0f);
	glDepthFunc(GL_LEQUAL);
	glEnable(GL_DEPTH_TEST);
	glHint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST);

	glViewport(0,0,width,height);
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	glMatrixMode(GL_MODELVIEW);
}

void stop()
{
    //Destroy window
    SDL_DestroyWindow( window );

    //Quit SDL subsystems
    SDL_Quit();


}

void loop()
{
	bool out=false;
	while(!out)
	{
			int mx, my;
			SDL_GetMouseState(&mx,&my);
			AssortedWidgets::UI::getSingleton().mouseMotion(mx,my);

			SDL_Event event;
			while(SDL_PollEvent(&event))
			{
				switch(event.type)
				{
					case SDL_QUIT:
					{
						break;
					}
					case SDL_MOUSEBUTTONUP:
					{
						AssortedWidgets::UI::getSingleton().importMouseRelease(event.button.button,event.button.x,event.button.y);
						break;
					}
					case SDL_MOUSEBUTTONDOWN:
					{
						AssortedWidgets::UI::getSingleton().importMousePress(event.button.button,event.button.x,event.button.y);
						break;
					}
					case SDL_KEYDOWN:
					{
                        AssortedWidgets::UI::getSingleton().importKeyDown(event.key.keysym.sym,event.key.keysym.mod);
						break;
					}
					case SDL_KEYUP:
					{
						//out=true;
                        AssortedWidgets::UI::getSingleton().importKeyUp(event.key.keysym.sym,event.key.keysym.mod);
						break;
					}
				}
			}


		AssortedWidgets::UI::getSingleton().paint();
        SDL_GL_SwapWindow( window );
	}
}

int main(int argc, char* argv [])
{
 //   QApplication application(argc, argv);
    int width=800;
    int height=600;
	init(width,height);
	AssortedWidgets::UI::getSingleton().init(width,height);
	//AssortedWidgets::UI::getSingleton().setQuitFunction(&stop);
	loop();
	stop();
	return 0;
}
